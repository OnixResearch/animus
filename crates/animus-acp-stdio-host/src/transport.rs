//! Thin bounded stdio transport shell.

#![allow(missing_docs)]

use std::io::BufRead;

use crate::codec::{
    decode_acp_frame, decode_permission_response, encode_acp_message, permission_request_message,
};
#[cfg(test)]
use crate::identities::{AcpCorrelationId, AcpSessionId};
use crate::model::{
    AcpCleanupStatus, AcpClientPermissionDecision, AcpClientPermissionRequest, AcpDecodedRequest,
    AcpFailureClass, AcpHostMessage, AcpTransportProfile, AcpTransportStatus,
};

const NEWLINE_BYTE: u8 = b'\n';
const INITIAL_LINE_CAPACITY_BYTES: usize = 4_096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AcpTransportReport {
    pub status: AcpTransportStatus,
    pub failure: Option<AcpFailureClass>,
    pub cleanup: AcpCleanupStatus,
    pub frame_count: u32,
    pub permission_count: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpTransportError {
    report: AcpTransportReport,
}

impl AcpTransportError {
    const fn new(failure: AcpFailureClass) -> Self {
        Self {
            report: failure_report(failure, AcpCleanupStatus::NotRequired, 0, 0),
        }
    }

    const fn from_report(report: AcpTransportReport) -> Self {
        Self { report }
    }

    #[must_use]
    pub const fn failure(&self) -> AcpFailureClass {
        match self.report.failure {
            Some(failure) => failure,
            None => AcpFailureClass::CleanupFailed,
        }
    }

    #[must_use]
    pub const fn report(&self) -> AcpTransportReport {
        self.report
    }
}

impl core::fmt::Display for AcpTransportError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "ACP stdio transport failed: {}",
            self.failure().as_str()
        )
    }
}

impl std::error::Error for AcpTransportError {}

pub trait AcpTransportSignal {
    fn cancellation_requested(&mut self) -> bool;
    fn deadline_exceeded(&mut self) -> bool;
}

#[derive(Default)]
pub struct NoopTransportSignal;

impl AcpTransportSignal for NoopTransportSignal {
    fn cancellation_requested(&mut self) -> bool {
        false
    }

    fn deadline_exceeded(&mut self) -> bool {
        false
    }
}

pub trait AcpDiagnosticSink {
    fn observe(&mut self, failure: AcpFailureClass);
}

pub struct StderrDiagnosticSink<Writer> {
    writer: Writer,
}

impl<Writer> StderrDiagnosticSink<Writer> {
    #[must_use]
    pub const fn new(writer: Writer) -> Self {
        Self { writer }
    }
}

impl<Writer: std::io::Write> AcpDiagnosticSink for StderrDiagnosticSink<Writer> {
    fn observe(&mut self, failure: AcpFailureClass) {
        let _result = writeln!(self.writer, "animus ACP host: {}", failure.as_str());
    }
}

pub trait AcpPeer {
    /// # Errors
    /// Returns a typed transport error when bounded encoding, writing, or flushing fails.
    fn send(&mut self, message: &AcpHostMessage) -> Result<(), AcpTransportError>;

    /// # Errors
    /// Returns a typed transport error when confirmation cannot be obtained safely.
    fn request_permission(
        &mut self,
        request: &AcpClientPermissionRequest,
    ) -> Result<AcpClientPermissionDecision, AcpTransportError>;
}

pub trait AcpStdioService {
    /// # Errors
    /// Returns a failure class when the request cannot be reduced or delegated safely.
    fn handle(
        &mut self,
        request: AcpDecodedRequest,
        peer: &mut dyn AcpPeer,
    ) -> Result<Vec<AcpHostMessage>, AcpFailureClass>;

    fn accepts_eof(&self) -> bool;

    /// # Errors
    /// Returns a failure class when host-owned resources cannot be released.
    fn cleanup(&mut self) -> Result<(), AcpFailureClass>;
}

pub struct AcpTransportIo<'a, Reader, Writer, Signal, Diagnostics> {
    reader: &'a mut Reader,
    writer: &'a mut Writer,
    signal: &'a mut Signal,
    diagnostics: &'a mut Diagnostics,
}

impl<'a, Reader, Writer, Signal, Diagnostics>
    AcpTransportIo<'a, Reader, Writer, Signal, Diagnostics>
{
    #[must_use]
    pub const fn new(
        reader: &'a mut Reader,
        writer: &'a mut Writer,
        signal: &'a mut Signal,
        diagnostics: &'a mut Diagnostics,
    ) -> Self {
        Self {
            reader,
            writer,
            signal,
            diagnostics,
        }
    }
}

/// # Errors
/// Returns a typed transport error for unsafe input, failed output, cancellation, or cleanup.
pub fn run_acp_stdio<Service>(
    service: &mut Service,
) -> Result<AcpTransportReport, AcpTransportError>
where
    Service: AcpStdioService,
{
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let stderr = std::io::stderr();
    let mut input = stdin.lock();
    let mut output = stdout.lock();
    let mut signal = NoopTransportSignal;
    let mut diagnostics = StderrDiagnosticSink::new(stderr.lock());
    let mut io = AcpTransportIo::new(&mut input, &mut output, &mut signal, &mut diagnostics);
    run_acp_transport(&mut io, service, AcpTransportProfile::reviewed())
}

/// # Errors
/// Returns a typed transport error for unsafe input, failed output, cancellation, or cleanup.
// r[impl agent_harness_runtime.acp_stdio_host.transport]
pub fn run_acp_transport<Reader, Writer, Service, Signal, Diagnostics>(
    io: &mut AcpTransportIo<'_, Reader, Writer, Signal, Diagnostics>,
    service: &mut Service,
    profile: AcpTransportProfile,
) -> Result<AcpTransportReport, AcpTransportError>
where
    Reader: BufRead,
    Writer: std::io::Write,
    Service: AcpStdioService,
    Signal: AcpTransportSignal,
    Diagnostics: AcpDiagnosticSink,
{
    let mut peer = StdioPeer {
        reader: &mut *io.reader,
        writer: &mut *io.writer,
        signal: &mut *io.signal,
        diagnostics: &mut *io.diagnostics,
        profile,
        frame_count: 0,
        permission_count: 0,
    };
    loop {
        if let Err(error) = peer.observe_signals() {
            return fail_with_cleanup(service, error.failure(), peer.counts(), peer.diagnostics);
        }
        let frame = match peer.read_frame(false) {
            Ok(frame) => frame,
            Err(error) => {
                return fail_with_cleanup(
                    service,
                    error.failure(),
                    peer.counts(),
                    peer.diagnostics,
                );
            }
        };
        let Some(frame) = frame else {
            if service.accepts_eof() {
                return cleanup(service, peer.counts(), peer.diagnostics);
            }
            return fail_with_cleanup(
                service,
                AcpFailureClass::InputEof,
                peer.counts(),
                peer.diagnostics,
            );
        };
        let request = match decode_acp_frame(frame.as_slice()) {
            Ok(request) => request,
            Err(error) => {
                return fail_with_cleanup(service, error.failure, peer.counts(), peer.diagnostics);
            }
        };
        let messages = match service.handle(request, &mut peer) {
            Ok(messages) => messages,
            Err(failure) => {
                return fail_with_cleanup(service, failure, peer.counts(), peer.diagnostics);
            }
        };
        for message in messages {
            if let Err(error) = peer.send(&message) {
                return fail_with_cleanup(
                    service,
                    error.failure(),
                    peer.counts(),
                    peer.diagnostics,
                );
            }
        }
    }
}

#[derive(Clone, Copy)]
struct TransportCounts {
    frame_count: u32,
    permission_count: u32,
}

struct StdioPeer<'a, Reader, Writer, Signal, Diagnostics> {
    reader: &'a mut Reader,
    writer: &'a mut Writer,
    signal: &'a mut Signal,
    diagnostics: &'a mut Diagnostics,
    profile: AcpTransportProfile,
    frame_count: u32,
    permission_count: u32,
}

impl<Reader, Writer, Signal, Diagnostics> StdioPeer<'_, Reader, Writer, Signal, Diagnostics>
where
    Reader: BufRead,
    Writer: std::io::Write,
    Signal: AcpTransportSignal,
    Diagnostics: AcpDiagnosticSink,
{
    const fn counts(&self) -> TransportCounts {
        TransportCounts {
            frame_count: self.frame_count,
            permission_count: self.permission_count,
        }
    }

    fn read_frame(&mut self, confirmation: bool) -> Result<Option<Vec<u8>>, AcpTransportError> {
        self.observe_signals()?;
        if self.frame_count >= self.profile.frames_per_connection {
            return self.fail(AcpFailureClass::FrameLimitExceeded);
        }
        let frame = read_bounded_line(self.reader, self.profile.frame_bytes)
            .map_err(AcpTransportError::new)?;
        if frame.is_none() && confirmation {
            return self.fail(AcpFailureClass::ConfirmationEof);
        }
        if frame.is_some() {
            self.frame_count = self
                .frame_count
                .saturating_add(crate::model::ACP_COUNTER_INCREMENT);
        }
        Ok(frame)
    }

    fn observe_signals(&mut self) -> Result<(), AcpTransportError> {
        if self.signal.deadline_exceeded() {
            return self.fail(AcpFailureClass::DeadlineExceeded);
        }
        if self.signal.cancellation_requested() {
            return self.fail(AcpFailureClass::CancellationAmbiguous);
        }
        Ok(())
    }

    fn fail<T>(&mut self, failure: AcpFailureClass) -> Result<T, AcpTransportError> {
        self.diagnostics.observe(failure);
        Err(AcpTransportError::new(failure))
    }
}

impl<Reader, Writer, Signal, Diagnostics> AcpPeer
    for StdioPeer<'_, Reader, Writer, Signal, Diagnostics>
where
    Reader: BufRead,
    Writer: std::io::Write,
    Signal: AcpTransportSignal,
    Diagnostics: AcpDiagnosticSink,
{
    fn send(&mut self, message: &AcpHostMessage) -> Result<(), AcpTransportError> {
        self.observe_signals()?;
        let mut encoded = encode_acp_message(message).map_err(|error| {
            self.diagnostics.observe(error.failure);
            AcpTransportError::new(error.failure)
        })?;
        encoded.push(NEWLINE_BYTE);
        if encoded.len() > self.profile.frame_bytes {
            return self.fail(AcpFailureClass::Backpressure);
        }
        write_frame(self.writer, encoded.as_slice()).map_err(|failure| {
            self.diagnostics.observe(failure);
            AcpTransportError::new(failure)
        })?;
        self.writer.flush().map_err(|_error| {
            self.diagnostics.observe(AcpFailureClass::FlushFailure);
            AcpTransportError::new(AcpFailureClass::FlushFailure)
        })
    }

    fn request_permission(
        &mut self,
        request: &AcpClientPermissionRequest,
    ) -> Result<AcpClientPermissionDecision, AcpTransportError> {
        if self.permission_count >= self.profile.permission_requests_per_connection {
            return self.fail(AcpFailureClass::Backpressure);
        }
        let message = permission_request_message(request).map_err(|error| {
            self.diagnostics.observe(error.failure);
            AcpTransportError::new(error.failure)
        })?;
        self.send(&message)?;
        self.permission_count = self
            .permission_count
            .saturating_add(crate::model::ACP_COUNTER_INCREMENT);
        let frame = self
            .read_frame(true)?
            .ok_or_else(|| AcpTransportError::new(AcpFailureClass::ConfirmationEof))?;
        decode_permission_response(request.request_id.as_str(), frame.as_slice()).map_err(|error| {
            self.diagnostics.observe(error.failure);
            AcpTransportError::new(error.failure)
        })
    }
}

fn read_bounded_line<Reader: BufRead>(
    reader: &mut Reader,
    max_frame_bytes: usize,
) -> Result<Option<Vec<u8>>, AcpFailureClass> {
    let capacity = max_frame_bytes.min(INITIAL_LINE_CAPACITY_BYTES);
    let mut line = Vec::with_capacity(capacity);
    loop {
        let available = reader
            .fill_buf()
            .map_err(|_error| AcpFailureClass::ReadFailure)?;
        if available.is_empty() {
            return if line.is_empty() {
                Ok(None)
            } else {
                Err(AcpFailureClass::InputEof)
            };
        }
        let newline = available.iter().position(|byte| *byte == NEWLINE_BYTE);
        let consumed = newline.map_or(available.len(), |index| index.saturating_add(1));
        if line.len().saturating_add(consumed) > max_frame_bytes {
            return Err(AcpFailureClass::OversizedFrame);
        }
        line.extend_from_slice(&available[..consumed]);
        reader.consume(consumed);
        if newline.is_some() {
            line.pop();
            return Ok(Some(line));
        }
    }
}

fn write_frame<Writer: std::io::Write>(
    writer: &mut Writer,
    frame: &[u8],
) -> Result<(), AcpFailureClass> {
    writer.write_all(frame).map_err(|error| match error.kind() {
        std::io::ErrorKind::BrokenPipe => AcpFailureClass::BrokenPipe,
        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut => {
            AcpFailureClass::Backpressure
        }
        _other => AcpFailureClass::WriteFailure,
    })
}

fn cleanup<Service: AcpStdioService, Diagnostics: AcpDiagnosticSink>(
    service: &mut Service,
    counts: TransportCounts,
    diagnostics: &mut Diagnostics,
) -> Result<AcpTransportReport, AcpTransportError> {
    if service.cleanup().is_err() {
        diagnostics.observe(AcpFailureClass::CleanupFailed);
        let report = failure_report(
            AcpFailureClass::CleanupFailed,
            AcpCleanupStatus::Failed,
            counts.frame_count,
            counts.permission_count,
        );
        return Err(AcpTransportError::from_report(report));
    }
    Ok(AcpTransportReport {
        status: AcpTransportStatus::CleanEof,
        failure: None,
        cleanup: AcpCleanupStatus::Completed,
        frame_count: counts.frame_count,
        permission_count: counts.permission_count,
    })
}

fn fail_with_cleanup<Service: AcpStdioService, Diagnostics: AcpDiagnosticSink>(
    service: &mut Service,
    failure: AcpFailureClass,
    counts: TransportCounts,
    diagnostics: &mut Diagnostics,
) -> Result<AcpTransportReport, AcpTransportError> {
    diagnostics.observe(failure);
    let cleanup = if service.cleanup().is_err() {
        diagnostics.observe(AcpFailureClass::CleanupFailed);
        AcpCleanupStatus::Failed
    } else {
        AcpCleanupStatus::Completed
    };
    Err(AcpTransportError::from_report(failure_report(
        failure,
        cleanup,
        counts.frame_count,
        counts.permission_count,
    )))
}

const fn failure_report(
    failure: AcpFailureClass,
    cleanup: AcpCleanupStatus,
    frame_count: u32,
    permission_count: u32,
) -> AcpTransportReport {
    AcpTransportReport {
        status: AcpTransportStatus::Failed,
        failure: Some(failure),
        cleanup,
        frame_count,
        permission_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INITIALIZE_FRAME: &str = "{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{\"protocolVersion\":1,\"clientCapabilities\":{}}}\n";
    const PERMISSION_ALLOW_FRAME: &str = "{\"jsonrpc\":\"2.0\",\"id\":\"permission.fixture\",\"result\":{\"outcome\":{\"outcome\":\"selected\",\"optionId\":\"allow_once\"}}}\n";
    const PERMISSION_STALE_FRAME: &str = "{\"jsonrpc\":\"2.0\",\"id\":\"permission.stale\",\"result\":{\"outcome\":{\"outcome\":\"selected\",\"optionId\":\"allow_once\"}}}\n";
    const MALFORMED_FRAME: &str = "{not-json}\n";
    const TRUNCATED_FRAME: &str = "{\"jsonrpc\":\"2.0\"";
    const PERMISSION_EXCHANGE_FRAME_COUNT: u32 = 2;

    #[derive(Default)]
    struct FixtureService {
        handled: u32,
        cleaned: bool,
        cleanup_failure: bool,
    }

    impl AcpStdioService for FixtureService {
        fn handle(
            &mut self,
            request: AcpDecodedRequest,
            _peer: &mut dyn AcpPeer,
        ) -> Result<Vec<AcpHostMessage>, AcpFailureClass> {
            self.handled = self
                .handled
                .saturating_add(crate::model::ACP_COUNTER_INCREMENT);
            let correlation = request
                .correlation_id()
                .ok_or(AcpFailureClass::UnknownCorrelation)?;
            Ok(vec![AcpHostMessage::Response {
                correlation_id: String::from(correlation),
                payload: serde_json::json!({"ok": true}),
            }])
        }

        fn accepts_eof(&self) -> bool {
            self.handled > 0
        }

        fn cleanup(&mut self) -> Result<(), AcpFailureClass> {
            self.cleaned = true;
            if self.cleanup_failure {
                Err(AcpFailureClass::CleanupFailed)
            } else {
                Ok(())
            }
        }
    }

    #[derive(Default)]
    struct PermissionService {
        handled: bool,
        cleaned: bool,
    }

    impl AcpStdioService for PermissionService {
        fn handle(
            &mut self,
            request: AcpDecodedRequest,
            peer: &mut dyn AcpPeer,
        ) -> Result<Vec<AcpHostMessage>, AcpFailureClass> {
            self.handled = true;
            let correlation = request
                .correlation_id()
                .ok_or(AcpFailureClass::UnknownCorrelation)?;
            let decision = peer
                .request_permission(&AcpClientPermissionRequest {
                    request_id: String::from("permission.fixture"),
                    session_id: AcpSessionId::from_admitted(String::from("session.fixture")),
                    tool_name: String::from("fixture_tool"),
                    tool_correlation_identity: AcpCorrelationId::from_admitted(String::from(
                        "tool.fixture",
                    )),
                })
                .map_err(|error| error.failure())?;
            if decision != AcpClientPermissionDecision::AllowOnce {
                return Err(AcpFailureClass::PermissionDenied);
            }
            Ok(vec![AcpHostMessage::Response {
                correlation_id: String::from(correlation),
                payload: serde_json::json!({"permission": "observed"}),
            }])
        }

        fn accepts_eof(&self) -> bool {
            self.handled
        }

        fn cleanup(&mut self) -> Result<(), AcpFailureClass> {
            self.cleaned = true;
            Ok(())
        }
    }

    struct BrokenWriter;

    impl std::io::Write for BrokenWriter {
        fn write(&mut self, _buffer: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::from(std::io::ErrorKind::BrokenPipe))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct WouldBlockWriter;

    impl std::io::Write for WouldBlockWriter {
        fn write(&mut self, _buffer: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::from(std::io::ErrorKind::WouldBlock))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct Diagnostics(Vec<AcpFailureClass>);

    impl AcpDiagnosticSink for Diagnostics {
        fn observe(&mut self, failure: AcpFailureClass) {
            self.0.push(failure);
        }
    }

    #[test]
    fn fixture_client_uses_production_codec_and_clean_eof() {
        let mut input = std::io::Cursor::new(INITIALIZE_FRAME.as_bytes());
        let mut output = Vec::new();
        let mut service = FixtureService::default();
        let mut signal = NoopTransportSignal;
        let mut diagnostics = Diagnostics::default();

        let mut io = AcpTransportIo::new(&mut input, &mut output, &mut signal, &mut diagnostics);
        let report = run_acp_transport(&mut io, &mut service, AcpTransportProfile::reviewed())
            .expect("fixture exchange");

        assert!(service.cleaned);
        assert_eq!(report.status, AcpTransportStatus::CleanEof);
        assert_eq!(report.frame_count, 1);
        assert_eq!(report.permission_count, 0);
        assert_eq!(service.handled, 1);
        assert!(diagnostics.0.is_empty());
        let rendered = String::from_utf8(output).expect("UTF-8 output");
        assert!(rendered.contains("\"result\""));
    }

    #[test]
    fn frame_count_exhaustion_fails_before_additional_dispatch() {
        const SINGLE_FRAME_LIMIT: u32 = 1;
        let mut input =
            std::io::Cursor::new(format!("{INITIALIZE_FRAME}{INITIALIZE_FRAME}").into_bytes());
        let mut output = Vec::new();
        let mut service = FixtureService::default();
        let mut signal = NoopTransportSignal;
        let mut diagnostics = Diagnostics::default();
        let mut profile = AcpTransportProfile::reviewed();
        profile.frames_per_connection = SINGLE_FRAME_LIMIT;
        let mut io = AcpTransportIo::new(&mut input, &mut output, &mut signal, &mut diagnostics);
        let error = run_acp_transport(&mut io, &mut service, profile)
            .expect_err("frame-count exhaustion denied");

        assert_eq!(error.failure(), AcpFailureClass::FrameLimitExceeded);
        assert_eq!(service.handled, SINGLE_FRAME_LIMIT);
        assert!(service.cleaned);
    }

    #[test]
    fn oversized_frame_fails_before_service_dispatch() {
        const FIXTURE_FRAME_LIMIT: usize = 8;
        let profile = AcpTransportProfile {
            frame_bytes: FIXTURE_FRAME_LIMIT,
            ..AcpTransportProfile::reviewed()
        };
        let mut input = std::io::Cursor::new(INITIALIZE_FRAME.as_bytes());
        let mut output = Vec::new();
        let mut service = FixtureService::default();
        let mut signal = NoopTransportSignal;
        let mut diagnostics = Diagnostics::default();
        let mut io = AcpTransportIo::new(&mut input, &mut output, &mut signal, &mut diagnostics);
        let error =
            run_acp_transport(&mut io, &mut service, profile).expect_err("oversized frame denied");

        assert_eq!(error.failure(), AcpFailureClass::OversizedFrame);
        assert_eq!(service.handled, 0);
        assert!(service.cleaned);
    }

    #[test]
    // r[verify agent_harness_runtime.acp_stdio_host.transport]
    // r[verify agent_harness_runtime.acp_stdio_host.verification]
    fn permission_round_trip_uses_bounded_production_transport() {
        let input_text = format!("{INITIALIZE_FRAME}{PERMISSION_ALLOW_FRAME}");
        let mut input = std::io::Cursor::new(input_text.as_bytes());
        let mut output = Vec::new();
        let mut service = PermissionService::default();
        let mut signal = NoopTransportSignal;
        let mut diagnostics = Diagnostics::default();
        let mut io = AcpTransportIo::new(&mut input, &mut output, &mut signal, &mut diagnostics);

        let report = run_acp_transport(&mut io, &mut service, AcpTransportProfile::reviewed())
            .expect("permission exchange");

        assert!(service.cleaned);
        assert_eq!(report.frame_count, PERMISSION_EXCHANGE_FRAME_COUNT);
        assert_eq!(report.permission_count, 1);
        let rendered = String::from_utf8(output).expect("UTF-8 output");
        assert!(rendered.contains("session/request_permission"));
        assert!(rendered.contains("permission.fixture"));
    }

    #[test]
    fn permission_eof_and_stale_correlation_fail_closed() {
        for (input_text, expected) in [
            (
                String::from(INITIALIZE_FRAME),
                AcpFailureClass::ConfirmationEof,
            ),
            (
                format!("{INITIALIZE_FRAME}{PERMISSION_STALE_FRAME}"),
                AcpFailureClass::UnknownCorrelation,
            ),
        ] {
            let mut input = std::io::Cursor::new(input_text.as_bytes());
            let mut output = Vec::new();
            let mut service = PermissionService::default();
            let mut signal = NoopTransportSignal;
            let mut diagnostics = Diagnostics::default();
            let mut io =
                AcpTransportIo::new(&mut input, &mut output, &mut signal, &mut diagnostics);
            let error = run_acp_transport(&mut io, &mut service, AcpTransportProfile::reviewed())
                .expect_err("permission response denied");

            assert_eq!(error.failure(), expected);
            assert!(service.cleaned);
        }
    }

    #[test]
    fn malformed_and_truncated_frames_fail_before_dispatch() {
        for (input_text, expected) in [
            (MALFORMED_FRAME, AcpFailureClass::MalformedFrame),
            (TRUNCATED_FRAME, AcpFailureClass::InputEof),
        ] {
            let mut input = std::io::Cursor::new(input_text.as_bytes());
            let mut output = Vec::new();
            let mut service = FixtureService::default();
            let mut signal = NoopTransportSignal;
            let mut diagnostics = Diagnostics::default();
            let mut io =
                AcpTransportIo::new(&mut input, &mut output, &mut signal, &mut diagnostics);
            let error = run_acp_transport(&mut io, &mut service, AcpTransportProfile::reviewed())
                .expect_err("unsafe frame denied");

            assert_eq!(error.failure(), expected);
            assert_eq!(service.handled, 0);
            assert!(service.cleaned);
            assert!(output.is_empty());
        }
    }

    #[test]
    fn cleanup_failure_is_reported_alongside_primary_failure() {
        let mut input = std::io::Cursor::new(MALFORMED_FRAME.as_bytes());
        let mut output = Vec::new();
        let mut service = FixtureService {
            cleanup_failure: true,
            ..FixtureService::default()
        };
        let mut signal = NoopTransportSignal;
        let mut diagnostics = Diagnostics::default();
        let mut io = AcpTransportIo::new(&mut input, &mut output, &mut signal, &mut diagnostics);
        let error = run_acp_transport(&mut io, &mut service, AcpTransportProfile::reviewed())
            .expect_err("cleanup failure surfaced");

        assert_eq!(error.failure(), AcpFailureClass::MalformedFrame);
        assert_eq!(error.report().cleanup, AcpCleanupStatus::Failed);
        assert!(diagnostics.0.contains(&AcpFailureClass::MalformedFrame));
        assert!(diagnostics.0.contains(&AcpFailureClass::CleanupFailed));
    }

    #[test]
    fn blocked_output_is_classified_as_backpressure_without_buffering() {
        let mut input = std::io::Cursor::new(INITIALIZE_FRAME.as_bytes());
        let mut output = WouldBlockWriter;
        let mut service = FixtureService::default();
        let mut signal = NoopTransportSignal;
        let mut diagnostics = Diagnostics::default();
        let mut io = AcpTransportIo::new(&mut input, &mut output, &mut signal, &mut diagnostics);
        let error = run_acp_transport(&mut io, &mut service, AcpTransportProfile::reviewed())
            .expect_err("backpressure classified");

        assert_eq!(error.failure(), AcpFailureClass::Backpressure);
        assert_eq!(service.handled, 1);
        assert!(service.cleaned);
    }

    #[test]
    fn broken_pipe_is_classified_without_retry_or_fallback() {
        let mut input = std::io::Cursor::new(INITIALIZE_FRAME.as_bytes());
        let mut output = BrokenWriter;
        let mut service = FixtureService::default();
        let mut signal = NoopTransportSignal;
        let mut diagnostics = Diagnostics::default();
        let mut io = AcpTransportIo::new(&mut input, &mut output, &mut signal, &mut diagnostics);
        let error = run_acp_transport(&mut io, &mut service, AcpTransportProfile::reviewed())
            .expect_err("broken pipe classified");

        assert_eq!(error.failure(), AcpFailureClass::BrokenPipe);
        assert_eq!(service.handled, 1);
        assert!(service.cleaned);
    }

    #[test]
    fn cancellation_signal_is_classified_as_ambiguous_before_dispatch() {
        struct Cancelled;
        impl AcpTransportSignal for Cancelled {
            fn cancellation_requested(&mut self) -> bool {
                true
            }
            fn deadline_exceeded(&mut self) -> bool {
                false
            }
        }
        let mut input = std::io::Cursor::new(INITIALIZE_FRAME.as_bytes());
        let mut output = Vec::new();
        let mut service = FixtureService::default();
        let mut signal = Cancelled;
        let mut diagnostics = Diagnostics::default();
        let mut io = AcpTransportIo::new(&mut input, &mut output, &mut signal, &mut diagnostics);
        let error = run_acp_transport(&mut io, &mut service, AcpTransportProfile::reviewed())
            .expect_err("cancellation ambiguity denied");

        assert_eq!(error.failure(), AcpFailureClass::CancellationAmbiguous);
        assert_eq!(service.handled, 0);
        assert!(service.cleaned);
    }

    #[test]
    fn deadline_signal_fails_before_reading_or_writing() {
        struct Expired;
        impl AcpTransportSignal for Expired {
            fn cancellation_requested(&mut self) -> bool {
                false
            }
            fn deadline_exceeded(&mut self) -> bool {
                true
            }
        }
        let mut input = std::io::Cursor::new(INITIALIZE_FRAME.as_bytes());
        let mut output = Vec::new();
        let mut service = FixtureService::default();
        let mut signal = Expired;
        let mut diagnostics = Diagnostics::default();
        let mut io = AcpTransportIo::new(&mut input, &mut output, &mut signal, &mut diagnostics);
        let error = run_acp_transport(&mut io, &mut service, AcpTransportProfile::reviewed())
            .expect_err("deadline denied");

        assert_eq!(error.failure(), AcpFailureClass::DeadlineExceeded);
        assert_eq!(service.handled, 0);
    }
}
