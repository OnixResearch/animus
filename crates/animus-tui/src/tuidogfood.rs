#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SmokeErrorKind(&'static str);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct HashDomain(&'static str);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RequiredMarker(&'static str);

const ARG_TERMINAL_UNAVAILABLE: &str = "--terminal-unavailable";
const ARG_CLEANUP_FAILURE: &str = "--cleanup-failure";
const ARG_OVERSIZED_RECEIPT: &str = "--oversized-receipt";
const STATUS_COMPLETED: &str = "completed";
const STATUS_FAILED: &str = "failed";
const RECORD_HEADER: &str = "ANIMUS_TUI_DOGFOOD_SMOKE";
const RECORD_NON_CLAIM: &str = "NON_CLAIM";
const RECEIPT_SCHEMA: &str = "animus.packaged-tui-dogfood-smoke.v1";
const EVIDENCE_RUNG: &str = "packaged-tui-test-backend";
const UPSTREAM_RUNG: &str = "packaged-headless-smoke";
const BACKEND_LABEL: &str = "ratatui-test-backend";
const SESSION_LABEL: &str = "packaged-tui-dogfood";
const AGENT_STATUS: &str = "completed";
const HEADLESS_HEADER: &str = "ANIMUS_DOGFOOD_SMOKE";
const HEADLESS_STATUS_MARKER: &str = "status=completed";
const HEADLESS_PACKAGED_RUNG_MARKER: &str = "packaged_rung=packaged-headless-smoke";
const HEADLESS_CORE_RUNG_MARKER: &str = "core_rung=fake-backed-integration";
const HEADLESS_EVENT_COUNT_PREFIX: &str = "event_count=";
const HEADLESS_EVENT_PREFIX: &str = "EVENT ";
const HEADLESS_TUI_NON_CLAIM: &str = "NON_CLAIM surface=tui";
const HASH_DOMAIN_HEADLESS: HashDomain =
    HashDomain("animus.packaged-tui-dogfood-smoke.headless-receipt.v1");
const HASH_DOMAIN_RENDER: HashDomain =
    HashDomain("animus.packaged-tui-dogfood-smoke.rendered-buffer.v1");
const HASH_SEPARATOR: &str = "|";
const ERROR_ARGUMENT_LIMIT: SmokeErrorKind = SmokeErrorKind("argument-limit");
const ERROR_TERMINAL_UNAVAILABLE: SmokeErrorKind = SmokeErrorKind("terminal-unavailable");
const ERROR_HEADLESS_SMOKE: SmokeErrorKind = SmokeErrorKind("headless-smoke");
const ERROR_RECEIPT_OVERSIZED: SmokeErrorKind = SmokeErrorKind("receipt-oversized");
const ERROR_RECEIPT_MALFORMED: SmokeErrorKind = SmokeErrorKind("receipt-malformed");
const ERROR_EVENT_COUNT: SmokeErrorKind = SmokeErrorKind("event-count-invalid");
const ERROR_UNSAFE_RECEIPT: SmokeErrorKind = SmokeErrorKind("unsafe-receipt");
const ERROR_BACKEND_CREATE: SmokeErrorKind = SmokeErrorKind("backend-create");
const ERROR_BACKEND_DRAW: SmokeErrorKind = SmokeErrorKind("backend-draw");
const ERROR_RENDER_OVERSIZED: SmokeErrorKind = SmokeErrorKind("render-oversized");
const ERROR_RENDER_INCOMPLETE: SmokeErrorKind = SmokeErrorKind("render-incomplete");
const ERROR_CLEANUP_FAILURE: SmokeErrorKind = SmokeErrorKind("cleanup-failure");
const HEADLESS_FAILURE_REJECTED: &str = "rejected";
const HEADLESS_MISSING_PROMPT: &str = "missing-prompt";
const HEADLESS_MISSING_MODEL: &str = "missing-model-service";
const HEADLESS_DENIED_CAPABILITY: &str = "denied-capability";
const HEADLESS_MALFORMED_OUTPUT: &str = "malformed-output";
const HEADLESS_UNSAFE_EVIDENCE: &str = "unsafe-evidence";
const MAX_ARGUMENT_COUNT: usize = 32;
const MAX_ARGUMENT_BYTES: usize = 4_096;
const MAX_HEADLESS_RECEIPT_BYTES: usize = 16_384;
const MAX_RENDERED_BUFFER_BYTES: usize = 8_192;
const MAX_EVENT_COUNT: u64 = 64;
const TEST_BACKEND_WIDTH_CELLS: u16 = 72;
const TEST_BACKEND_HEIGHT_CELLS: u16 = 9;
const RECEIPT_FIXED_RECORD_COUNT: usize = 2;
const OVERSIZED_RECEIPT_BYTES: usize = MAX_HEADLESS_RECEIPT_BYTES.saturating_add(1);
const NON_CLAIMS: &[&str] = &[
    "live-terminal",
    "raw-mode",
    "input-polling",
    "daemon-attach",
    "live-provider",
    "plugin-runtime",
    "production-readiness",
    "release-readiness",
    "clankers-parity",
];
const UNSAFE_RECEIPT_MARKERS: &[&str] = &[
    "RAW_PROMPT",
    "provider_payload",
    "/home/",
    "BEGIN PRIVATE KEY",
    "TOKEN=",
    "PASSWORD=",
];
const SAFE_HEADLESS_FAILURES: &[&str] = &[
    HEADLESS_MISSING_PROMPT,
    HEADLESS_MISSING_MODEL,
    HEADLESS_DENIED_CAPABILITY,
    HEADLESS_MALFORMED_OUTPUT,
    HEADLESS_UNSAFE_EVIDENCE,
];

const _: () = assert!(TEST_BACKEND_WIDTH_CELLS > 0);
const _: () = assert!(TEST_BACKEND_HEIGHT_CELLS > 0);
const _: () = assert!(MAX_ARGUMENT_COUNT > 0);
const _: () = assert!(MAX_ARGUMENT_BYTES > 0);
const _: () = assert!(MAX_HEADLESS_RECEIPT_BYTES > 0);
const _: () = assert!(MAX_RENDERED_BUFFER_BYTES > 0);
const _: () = assert!(MAX_EVENT_COUNT > 0);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BackendAvailability {
    Available,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CleanupObservation {
    Complete,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SmokeOptions {
    headless_args: Vec<String>,
    backend: BackendAvailability,
    cleanup: CleanupObservation,
    is_receipt_fault_requested: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct TuiProjection {
    event_count: u64,
    headless_receipt_hash: String,
    shell_snapshot: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RenderObservation {
    rendered_buffer_hash: String,
}

/// Runs the non-default packaged TUI dogfood smoke from already-loaded arguments.
///
/// The function composes the accepted packaged headless smoke with a fixed Ratatui
/// `TestBackend`. The returned evidence omits the prompt, model output, tool output,
/// complete headless receipt, and rendered buffer.
///
/// # Errors
///
/// Returns a stable error when host admission, headless execution, bounded projection,
/// deterministic rendering, or cleanup evidence fails.
pub fn run_packaged_tui_dogfood_smoke(args: &[String]) -> Result<String, String> {
    let options = parse_options(args)?;
    admit_backend(options.backend)?;

    let headless_receipt = run_headless_smoke(&options)?;
    let projection = project_headless_receipt(&headless_receipt)?;
    let render_observation = render_test_backend(&projection.shell_snapshot)?;
    admit_cleanup(options.cleanup)?;

    Ok(render_report(&projection, &render_observation))
}

fn parse_options(args: &[String]) -> Result<SmokeOptions, String> {
    validate_arguments(args)?;
    debug_assert!(args.len() <= MAX_ARGUMENT_COUNT);
    debug_assert!(
        args.iter()
            .all(|argument| argument.len() <= MAX_ARGUMENT_BYTES)
    );

    let mut headless_args = Vec::with_capacity(args.len());
    let mut backend = BackendAvailability::Available;
    let mut cleanup = CleanupObservation::Complete;
    let mut is_receipt_fault_requested = false;

    for argument in args {
        match argument.as_str() {
            ARG_TERMINAL_UNAVAILABLE => backend = BackendAvailability::Unavailable,
            ARG_CLEANUP_FAILURE => cleanup = CleanupObservation::Failed,
            ARG_OVERSIZED_RECEIPT => is_receipt_fault_requested = true,
            _other => headless_args.push(argument.clone()),
        }
    }

    Ok(SmokeOptions {
        headless_args,
        backend,
        cleanup,
        is_receipt_fault_requested,
    })
}

fn validate_arguments(args: &[String]) -> Result<(), String> {
    if args.len() > MAX_ARGUMENT_COUNT {
        return Err(format_error(ERROR_ARGUMENT_LIMIT, "count"));
    }

    let mut total_bytes = 0_usize;
    for argument in args {
        total_bytes = total_bytes
            .checked_add(argument.len())
            .ok_or_else(|| format_error(ERROR_ARGUMENT_LIMIT, "overflow"))?;
        if total_bytes > MAX_ARGUMENT_BYTES {
            return Err(format_error(ERROR_ARGUMENT_LIMIT, "bytes"));
        }
    }
    debug_assert!(total_bytes <= MAX_ARGUMENT_BYTES);
    Ok(())
}

fn admit_backend(backend: BackendAvailability) -> Result<(), String> {
    match backend {
        BackendAvailability::Available => Ok(()),
        BackendAvailability::Unavailable => {
            Err(format_error(ERROR_TERMINAL_UNAVAILABLE, BACKEND_LABEL))
        }
    }
}

fn run_headless_smoke(options: &SmokeOptions) -> Result<String, String> {
    if options.is_receipt_fault_requested {
        return Ok("x".repeat(OVERSIZED_RECEIPT_BYTES));
    }

    animus_std::run_packaged_headless_dogfood_smoke(&options.headless_args)
        .map_err(|error| safe_headless_error(&error))
}

fn project_headless_receipt(headless_receipt: &str) -> Result<TuiProjection, String> {
    validate_receipt_size(headless_receipt)?;
    validate_receipt_safety(headless_receipt)?;
    let event_count = validate_headless_header(headless_receipt)?;
    validate_event_rows(headless_receipt, event_count)?;

    let shell_snapshot = build_shell_snapshot(event_count);
    debug_assert!(!shell_snapshot.is_empty());
    debug_assert!(event_count > 0);

    Ok(TuiProjection {
        event_count,
        headless_receipt_hash: digest(HASH_DOMAIN_HEADLESS, headless_receipt),
        shell_snapshot,
    })
}

fn validate_receipt_size(receipt: &str) -> Result<(), String> {
    if receipt.is_empty() {
        return Err(format_error(ERROR_RECEIPT_MALFORMED, "empty"));
    }
    if receipt.len() > MAX_HEADLESS_RECEIPT_BYTES {
        return Err(format_error(ERROR_RECEIPT_OVERSIZED, "byte-limit"));
    }
    Ok(())
}

fn validate_receipt_safety(receipt: &str) -> Result<(), String> {
    for marker in UNSAFE_RECEIPT_MARKERS {
        if receipt.contains(marker) {
            return Err(format_error(ERROR_UNSAFE_RECEIPT, "unsafe-marker"));
        }
    }
    if receipt
        .chars()
        .any(|character| character.is_control() && character != '\n')
    {
        return Err(format_error(ERROR_UNSAFE_RECEIPT, "non-printing-character"));
    }
    Ok(())
}

fn validate_headless_header(receipt: &str) -> Result<u64, String> {
    let header = receipt
        .lines()
        .next()
        .ok_or_else(|| format_error(ERROR_RECEIPT_MALFORMED, "missing-header"))?;
    if !header.starts_with(HEADLESS_HEADER) {
        return Err(format_error(ERROR_RECEIPT_MALFORMED, "wrong-header"));
    }
    require_receipt_marker(header, RequiredMarker(HEADLESS_STATUS_MARKER))?;
    require_receipt_marker(header, RequiredMarker(HEADLESS_PACKAGED_RUNG_MARKER))?;
    require_receipt_marker(header, RequiredMarker(HEADLESS_CORE_RUNG_MARKER))?;
    require_receipt_marker(receipt, RequiredMarker(HEADLESS_TUI_NON_CLAIM))?;

    let event_count = header
        .split_ascii_whitespace()
        .find_map(|field| field.strip_prefix(HEADLESS_EVENT_COUNT_PREFIX))
        .ok_or_else(|| format_error(ERROR_EVENT_COUNT, "missing"))?
        .parse::<u64>()
        .map_err(|_error| format_error(ERROR_EVENT_COUNT, "malformed"))?;
    if event_count == 0 {
        return Err(format_error(ERROR_EVENT_COUNT, "zero"));
    }
    if event_count > MAX_EVENT_COUNT {
        return Err(format_error(ERROR_EVENT_COUNT, "out-of-range"));
    }
    debug_assert!(event_count > 0);
    debug_assert!(event_count <= MAX_EVENT_COUNT);
    Ok(event_count)
}

fn require_receipt_marker(receipt: &str, marker: RequiredMarker) -> Result<(), String> {
    if !receipt.contains(marker.0) {
        return Err(format_error(
            ERROR_RECEIPT_MALFORMED,
            "missing-required-marker",
        ));
    }
    Ok(())
}

fn validate_event_rows(receipt: &str, expected_count: u64) -> Result<(), String> {
    let observed_count = receipt
        .lines()
        .filter(|line| line.starts_with(HEADLESS_EVENT_PREFIX))
        .fold(0_u64, |count, _line| count.saturating_add(1));
    if observed_count != expected_count {
        return Err(format_error(ERROR_EVENT_COUNT, "row-mismatch"));
    }
    Ok(())
}

fn build_shell_snapshot(event_count: u64) -> String {
    let mut state = crate::AppShellState::new();
    state.set_terminal_state(crate::TerminalState::Ready);
    state.set_session_label(SESSION_LABEL);
    state.set_agent_status(AGENT_STATUS);
    state.set_transcript_entry_count(event_count);
    state.set_prompt_input("");
    crate::render_app_shell_snapshot(&state)
}

fn render_test_backend(snapshot: &str) -> Result<RenderObservation, String> {
    let backend =
        ratatui::backend::TestBackend::new(TEST_BACKEND_WIDTH_CELLS, TEST_BACKEND_HEIGHT_CELLS);
    let mut terminal = ratatui::Terminal::new(backend)
        .map_err(|_error| format_error(ERROR_BACKEND_CREATE, BACKEND_LABEL))?;

    terminal
        .draw(|frame| {
            let widget = ratatui::widgets::Paragraph::new(snapshot)
                .block(
                    ratatui::widgets::Block::new()
                        .title("Animus TUI Dogfood")
                        .borders(ratatui::widgets::Borders::ALL),
                )
                .wrap(ratatui::widgets::Wrap { trim: false });
            frame.render_widget(widget, frame.area());
        })
        .map_err(|_error| format_error(ERROR_BACKEND_DRAW, BACKEND_LABEL))?;

    let rendered = buffer_text(terminal.backend().buffer());
    if rendered.len() > MAX_RENDERED_BUFFER_BYTES {
        return Err(format_error(ERROR_RENDER_OVERSIZED, "byte-limit"));
    }
    require_render_marker(&rendered, RequiredMarker("Animus TUI Dogfood"))?;
    require_render_marker(&rendered, RequiredMarker(SESSION_LABEL))?;
    require_render_marker(&rendered, RequiredMarker(AGENT_STATUS))?;
    debug_assert!(rendered.contains(SESSION_LABEL));
    debug_assert!(rendered.contains(AGENT_STATUS));

    Ok(RenderObservation {
        rendered_buffer_hash: digest(HASH_DOMAIN_RENDER, &rendered),
    })
}

fn require_render_marker(rendered: &str, marker: RequiredMarker) -> Result<(), String> {
    if !rendered.contains(marker.0) {
        return Err(format_error(ERROR_RENDER_INCOMPLETE, "missing-marker"));
    }
    Ok(())
}

fn buffer_text(buffer: &ratatui::buffer::Buffer) -> String {
    let mut rows = Vec::with_capacity(usize::from(TEST_BACKEND_HEIGHT_CELLS));
    for y in 0..TEST_BACKEND_HEIGHT_CELLS {
        let mut row = String::with_capacity(usize::from(TEST_BACKEND_WIDTH_CELLS));
        for x in 0..TEST_BACKEND_WIDTH_CELLS {
            row.push_str(buffer[(x, y)].symbol());
        }
        rows.push(row.trim_end().to_owned());
    }
    rows.join("\n")
}

fn admit_cleanup(cleanup: CleanupObservation) -> Result<(), String> {
    match cleanup {
        CleanupObservation::Complete => Ok(()),
        CleanupObservation::Failed => Err(format_error(ERROR_CLEANUP_FAILURE, BACKEND_LABEL)),
    }
}

fn render_report(projection: &TuiProjection, render: &RenderObservation) -> String {
    let mut lines = Vec::with_capacity(NON_CLAIMS.len().saturating_add(RECEIPT_FIXED_RECORD_COUNT));
    lines.push(format!(
        "{RECORD_HEADER} schema={RECEIPT_SCHEMA} status={STATUS_COMPLETED} evidence_rung={EVIDENCE_RUNG} upstream_rung={UPSTREAM_RUNG} backend={BACKEND_LABEL} event_count={} viewport={}x{} cleanup=complete headless_receipt_hash={} rendered_buffer_hash={}",
        projection.event_count,
        TEST_BACKEND_WIDTH_CELLS,
        TEST_BACKEND_HEIGHT_CELLS,
        projection.headless_receipt_hash,
        render.rendered_buffer_hash,
    ));
    for surface in NON_CLAIMS {
        lines.push(format!("{RECORD_NON_CLAIM} surface={surface}"));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn digest(domain: HashDomain, value: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.0.as_bytes());
    hasher.update(HASH_SEPARATOR.as_bytes());
    hasher.update(value.as_bytes());
    hasher.finalize().to_hex().to_string()
}

fn safe_headless_error(error: &str) -> String {
    let safe_code = SAFE_HEADLESS_FAILURES
        .iter()
        .find(|code| error.contains(**code))
        .copied()
        .unwrap_or(HEADLESS_FAILURE_REJECTED);
    format_error(ERROR_HEADLESS_SMOKE, safe_code)
}

fn format_error(kind: SmokeErrorKind, detail: &str) -> String {
    format!("{STATUS_FAILED}:{}:{detail}", kind.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROMPT: &str = "bounded TUI prompt fixture";
    const TOOL_LABEL: &str = "fake-readonly-tool";
    const HASH_HEX_LENGTH: usize = 64;

    #[test]
    fn packaged_tui_smoke_renders_safe_metadata_only_evidence() {
        // r[verify agent_harness_runtime.packaged_tui_dogfood_smoke.contract]
        // r[verify agent_harness_runtime.packaged_tui_dogfood_smoke.receipt]
        let smoke_args = args(&["--prompt", PROMPT, "--tool", TOOL_LABEL]);
        let output =
            run_packaged_tui_dogfood_smoke(&smoke_args).expect("packaged TUI smoke must pass");
        let repeated_output = run_packaged_tui_dogfood_smoke(&smoke_args)
            .expect("repeated packaged TUI smoke must pass");

        assert_eq!(output, repeated_output);
        assert!(output.contains(RECORD_HEADER));
        assert!(output.contains("event_count="));
        assert!(output.contains("evidence_rung=packaged-tui-test-backend"));
        assert!(output.contains("upstream_rung=packaged-headless-smoke"));
        assert!(output.contains("backend=ratatui-test-backend"));
        assert!(output.contains("surface=live-terminal"));
        assert!(output.contains("surface=production-readiness"));
        assert!(!output.contains(PROMPT));
        assert!(!output.contains(TOOL_LABEL));
        assert!(!output.contains(HEADLESS_HEADER));
        assert!(!output.contains("terminal: app ready"));
    }

    #[test]
    fn argument_limits_fail_before_host_admission() {
        let too_many = vec![String::new(); MAX_ARGUMENT_COUNT.saturating_add(1)];
        let count_error =
            run_packaged_tui_dogfood_smoke(&too_many).expect_err("too many arguments must fail");
        assert!(count_error.contains(ERROR_ARGUMENT_LIMIT.0));
        assert!(count_error.contains("count"));

        let invalid_argument_fixture = vec!["x".repeat(MAX_ARGUMENT_BYTES.saturating_add(1))];
        let byte_error = run_packaged_tui_dogfood_smoke(&invalid_argument_fixture)
            .expect_err("oversized argument data must fail");
        assert!(byte_error.contains(ERROR_ARGUMENT_LIMIT.0));
        assert!(byte_error.contains("bytes"));
    }

    #[test]
    fn terminal_loss_fails_before_headless_prompt_validation() {
        // r[verify agent_harness_runtime.packaged_tui_dogfood_smoke.host_gate.terminal_loss]
        let error = run_packaged_tui_dogfood_smoke(&args(&[ARG_TERMINAL_UNAVAILABLE]))
            .expect_err("terminal loss must fail");

        assert!(error.contains(ERROR_TERMINAL_UNAVAILABLE.0));
        assert!(!error.contains(HEADLESS_MISSING_PROMPT));
    }

    #[test]
    fn capability_denial_is_safe_and_fail_closed() {
        // r[verify agent_harness_runtime.packaged_tui_dogfood_smoke.verification.focused_rail]
        let error =
            run_packaged_tui_dogfood_smoke(&args(&["--prompt", PROMPT, "--deny-capability"]))
                .expect_err("capability denial must fail");

        assert!(error.contains(ERROR_HEADLESS_SMOKE.0));
        assert!(error.contains(HEADLESS_DENIED_CAPABILITY));
        assert!(!error.contains(PROMPT));
    }

    #[test]
    fn oversized_receipt_and_cleanup_failure_block_success() {
        // r[verify agent_harness_runtime.packaged_tui_dogfood_smoke.projection.oversized]
        // r[verify agent_harness_runtime.packaged_tui_dogfood_smoke.host_gate.cleanup_failure]
        let receipt_error =
            run_packaged_tui_dogfood_smoke(&args(&["--prompt", PROMPT, ARG_OVERSIZED_RECEIPT]))
                .expect_err("oversized receipt must fail");
        assert!(receipt_error.contains(ERROR_RECEIPT_OVERSIZED.0));

        let cleanup =
            run_packaged_tui_dogfood_smoke(&args(&["--prompt", PROMPT, ARG_CLEANUP_FAILURE]))
                .expect_err("cleanup failure must fail");
        assert!(cleanup.contains(ERROR_CLEANUP_FAILURE.0));
        assert!(!cleanup.contains(RECORD_HEADER));
    }

    #[test]
    fn pure_projection_rejects_malformed_counts_and_unsafe_markers() {
        let malformed =
            project_headless_receipt("wrong header").expect_err("wrong header must fail");
        assert!(malformed.contains(ERROR_RECEIPT_MALFORMED.0));

        let malformed_count_receipt = format!(
            "{HEADLESS_HEADER} {HEADLESS_STATUS_MARKER} {HEADLESS_PACKAGED_RUNG_MARKER} {HEADLESS_CORE_RUNG_MARKER} event_count=invalid\n{HEADLESS_TUI_NON_CLAIM}"
        );
        let malformed_count_error = project_headless_receipt(&malformed_count_receipt)
            .expect_err("malformed event count must fail");
        assert!(malformed_count_error.contains(ERROR_EVENT_COUNT.0));

        let mismatched_count_receipt = format!(
            "{HEADLESS_HEADER} {HEADLESS_STATUS_MARKER} {HEADLESS_PACKAGED_RUNG_MARKER} {HEADLESS_CORE_RUNG_MARKER} event_count=2\nEVENT sequence=1\n{HEADLESS_TUI_NON_CLAIM}"
        );
        let mismatched_count_error = project_headless_receipt(&mismatched_count_receipt)
            .expect_err("mismatched event count must fail");
        assert!(mismatched_count_error.contains(ERROR_EVENT_COUNT.0));

        let unsafe_receipt = format!(
            "{HEADLESS_HEADER} {HEADLESS_STATUS_MARKER} {HEADLESS_PACKAGED_RUNG_MARKER} {HEADLESS_CORE_RUNG_MARKER} event_count=1\nEVENT sequence=1\n{HEADLESS_TUI_NON_CLAIM}\nRAW_PROMPT"
        );
        let unsafe_error =
            project_headless_receipt(&unsafe_receipt).expect_err("unsafe marker must fail");
        assert!(unsafe_error.contains(ERROR_UNSAFE_RECEIPT.0));

        let non_printing_receipt = format!(
            "{HEADLESS_HEADER} {HEADLESS_STATUS_MARKER} {HEADLESS_PACKAGED_RUNG_MARKER} {HEADLESS_CORE_RUNG_MARKER} event_count=1\nEVENT sequence=1\n{HEADLESS_TUI_NON_CLAIM}\0"
        );
        let non_printing_error = project_headless_receipt(&non_printing_receipt)
            .expect_err("non-printing receipt content must fail");
        assert!(non_printing_error.contains(ERROR_UNSAFE_RECEIPT.0));
    }

    #[test]
    fn test_backend_render_is_bounded_and_identified() {
        // r[verify agent_harness_runtime.packaged_tui_dogfood_smoke.projection.pass]
        let snapshot = build_shell_snapshot(1);
        let observation = render_test_backend(&snapshot).expect("test backend render must pass");
        let repeated_observation =
            render_test_backend(&snapshot).expect("repeated test backend render must pass");

        assert_eq!(observation, repeated_observation);
        assert_eq!(observation.rendered_buffer_hash.len(), HASH_HEX_LENGTH);
        assert_ne!(
            observation.rendered_buffer_hash,
            digest(HASH_DOMAIN_RENDER, "")
        );
    }

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }
}
