//! Optional `no_std` steerable external-agent harness contracts.
//!
//! This crate adapts the durable owner-gated command plane and bounded
//! external-agent harness boundary from Zeron (`github.com/zeronsh/comet`,
//! MIT) as a bounded reference. It owns pure contract DTOs, validation,
//! command-plane transitions, an implementation-free bounded external-process
//! service port, and metadata-only receipts. Hosts own subprocess execution,
//! persistence, transport, credentials, clocks, and all I/O authority.
//!
//! The bounded-reference exclusions are structural: no gpui UI, no Loro CRDT
//! sync, no TypeScript edge worker, no Cloudflare Durable Objects, no
//! token-usage display, and no multi-device transport live in reusable crates.

#![no_std]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

extern crate alloc;

use alloc::{format, string::String, vec::Vec};
use core::fmt;

/// The reviewed Zeron harness adaptation schema.
pub const SCHEMA: &str = "animus.zeron-harness.v1";
/// The reviewed external-agent harness cohort.
pub const REVIEWED_COHORT: &str = "zeronsh-comet-steerable-harness";
/// The reviewed adaptation source reference.
pub const REVIEWED_SOURCE: &str = "github.com/zeronsh/comet";
/// The reviewed adaptation source license.
pub const REVIEWED_LICENSE: &str = "MIT";
/// The command identity derivation domain.
pub const COMMAND_IDENTITY_DOMAIN: &str = "animus.zeron-harness-command-identity.v1";
/// The command payload hash domain.
pub const PAYLOAD_DOMAIN: &str = "animus.zeron-harness-payload.v1";
/// The plane transition domain retained for receipt linking.
pub const PLANE_DOMAIN: &str = "animus.zeron-harness-plane.v1";
/// The receipt identity derivation domain.
pub const RECEIPT_DOMAIN: &str = "animus.zeron-harness-receipt.v1";
/// The length of a hexadecimal BLAKE3 identity.
pub const BLAKE3_HEX_CHARS: usize = 64;
/// The largest accepted command, session, owner, and run identifier.
pub const MAX_IDENTIFIER_BYTES: usize = 256;
/// The largest accepted model label.
pub const MAX_MODEL_LABEL_BYTES: usize = 128;
/// The largest accepted per-option label.
pub const MAX_OPTION_LABEL_BYTES: usize = 128;
/// The largest accepted option catalog size.
pub const MAX_OPTION_LABELS: usize = 64;
/// The largest accepted raw command payload length.
pub const MAX_PAYLOAD_BYTES: usize = 262_144;
/// The smallest accepted raw command payload length.
pub const MIN_PAYLOAD_BYTES: usize = 1;
/// The smallest accepted command sequence.
pub const MIN_SEQUENCE: u64 = 1;
/// The largest retained processed-command ledger size.
pub const MAX_PROCESSED_ENTRIES: usize = 65_536;
/// The smallest allowed processed-command ledger size.
pub const MIN_PROCESSED_ENTRIES: usize = 1;
/// The largest accepted queued-command identity count.
pub const MAX_QUEUE_ENTRIES: usize = 65_536;
/// The largest accepted diagnostic and family label length.
pub const MAX_LABEL_BYTES: usize = 128;
/// The smallest accepted process limit value.
pub const MIN_LIMIT_UNITS: usize = 1;
/// The largest accepted bounded stream capacity.
pub const MAX_STREAM_BYTES: usize = 16_777_216;
/// The largest accepted bounded standard-input capacity.
pub const MAX_STDIN_BYTES: usize = 1_048_576;
/// The smallest accepted process window.
pub const MIN_PROCESS_WINDOW_MS: u64 = 1;
/// The largest accepted process window.
pub const MAX_PROCESS_WINDOW_MS: u64 = 86_400_000;
/// The smallest accepted teardown window.
pub const MIN_TEARDOWN_WINDOW_MS: u64 = 1;
/// The largest accepted teardown window.
pub const MAX_TEARDOWN_WINDOW_MS: u64 = 10_000;
/// The smallest accepted poll interval.
pub const MIN_POLL_INTERVAL_MS: u64 = 1;
/// The largest accepted poll interval.
pub const MAX_POLL_INTERVAL_MS: u64 = 1_000;
/// The largest accepted program path label.
pub const MAX_PROGRAM_BYTES: usize = 512;
/// The largest accepted argument list size.
pub const MAX_ARGS: usize = 128;
/// The largest accepted single argument length.
pub const MAX_ARG_BYTES: usize = 4_096;
/// The largest accepted environment entry count.
pub const MAX_ENV_ENTRIES: usize = 256;
/// The largest accepted environment key or value length.
pub const MAX_ENV_BYTES: usize = 4_096;
/// The reviewed harness family label.
pub const HARNESS_FAMILY: &str = "external-agent-subprocess";

const HASH_SEPARATOR: u8 = 0;
const HEX_ALPHABET: &[u8; 16] = b"0123456789abcdef";

/// A typed lifecycle or validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// A required field or limit is absent.
    MissingField,
    /// An identifier is empty.
    EmptyIdentifier,
    /// An identifier, label, or diagnostic exceeds its bound.
    ValueTooLong,
    /// The command kind is not recognized.
    UnknownKind,
    /// The referenced outcome is not recognized.
    UnknownOutcome,
    /// A command payload is empty.
    EmptyPayload,
    /// A command payload exceeds the bounded cap.
    PayloadTooLarge,
    /// A supplied payload does not match the command payload identity.
    PayloadHashMismatch,
    /// Identity material embeds raw body material instead of a hash.
    IdentityLeak,
    /// A command sequence is below the accepted minimum.
    SequenceZero,
    /// The option catalog exceeds its bound.
    TooManyOptions,
    /// A process limit is absent or below its minimum.
    MissingLimit,
    /// A process limit exceeds its bound.
    LimitTooLarge,
    /// A receipt or hash field is not a valid bounded hexadecimal identity.
    InvalidHash,
    /// A processed marker is not present in the processed-command ledger.
    StaleMarker,
    /// The processed-command ledger exceeds its configured cap.
    ProcessedLedgerOverflow,
    /// The command queue exceeds its configured cap.
    QueueOverflow,
    /// A receipt candidate carries raw material that must be redacted.
    RawMaterialLeak,
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::MissingField => "required field or limit is absent",
            Self::EmptyIdentifier => "identifier is empty",
            Self::ValueTooLong => "value exceeds its bound",
            Self::UnknownKind => "command kind is not recognized",
            Self::UnknownOutcome => "outcome is not recognized",
            Self::EmptyPayload => "command payload is empty",
            Self::PayloadTooLarge => "command payload exceeds its bound",
            Self::PayloadHashMismatch => "payload does not match the command payload identity",
            Self::IdentityLeak => "identity material embeds raw body material",
            Self::SequenceZero => "command sequence is below the minimum",
            Self::TooManyOptions => "option catalog exceeds its bound",
            Self::MissingLimit => "process limit is absent or below its minimum",
            Self::LimitTooLarge => "process limit exceeds its bound",
            Self::InvalidHash => "hash is not a valid bounded hexadecimal identity",
            Self::StaleMarker => "processed marker is absent from the ledger",
            Self::ProcessedLedgerOverflow => "processed-command ledger exceeds its cap",
            Self::QueueOverflow => "command queue exceeds its cap",
            Self::RawMaterialLeak => "receipt candidate carries raw material",
        };
        formatter.write_str(text)
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(char::from(HEX_ALPHABET[usize::from(byte >> 4)]));
        output.push(char::from(HEX_ALPHABET[usize::from(byte & 0x0f)]));
    }
    output
}

fn hex_decode(text: &str) -> Option<[u8; BLAKE3_HEX_CHARS / 2]> {
    if text.len() != BLAKE3_HEX_CHARS {
        return None;
    }
    let mut output = [0_u8; BLAKE3_HEX_CHARS / 2];
    for (index, byte) in output.iter_mut().enumerate() {
        let high = nibble(text.as_bytes()[index * 2])?;
        let low = nibble(text.as_bytes()[index * 2 + 1])?;
        *byte = (high << 4) | low;
    }
    Some(output)
}

fn nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    }
}

fn digest(domain: &str, material: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    hasher.update(&[HASH_SEPARATOR]);
    hasher.update(material);
    hex_encode(hasher.finalize().as_bytes())
}

pub(crate) fn validate_identifier(value: &str) -> Result<(), Error> {
    if value.is_empty() {
        return Err(Error::EmptyIdentifier);
    }
    if value.len() > MAX_IDENTIFIER_BYTES {
        return Err(Error::ValueTooLong);
    }
    Ok(())
}

fn validate_bounded_text(value: &str, bound: usize) -> Result<(), Error> {
    if value.len() > bound {
        return Err(Error::ValueTooLong);
    }
    Ok(())
}

fn validate_hash(value: &str) -> Result<(), Error> {
    if hex_decode(value).is_none() {
        return Err(Error::InvalidHash);
    }
    Ok(())
}

fn validate_bounded_count(value: usize, bound: usize, empty_error: Error) -> Result<(), Error> {
    if value < MIN_LIMIT_UNITS {
        return Err(empty_error);
    }
    if value > bound {
        return Err(Error::LimitTooLarge);
    }
    Ok(())
}

/// One durable external-agent control operation.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CommandKind {
    /// Send a new prompt or command to the harness.
    Send,
    /// Steer an active run with a follow-up.
    Steer,
    /// Interrupt the active run.
    Interrupt,
    /// Respond to a harness input request.
    RespondInput,
}

impl CommandKind {
    /// Returns the stable metadata label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Send => "send",
            Self::Steer => "steer",
            Self::Interrupt => "interrupt",
            Self::RespondInput => "respond_input",
        }
    }

    /// Parses a stable metadata label.
    #[must_use]
    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "send" => Some(Self::Send),
            "steer" => Some(Self::Steer),
            "interrupt" => Some(Self::Interrupt),
            "respond_input" => Some(Self::RespondInput),
            _ => None,
        }
    }
}

/// The reasoning visibility preference carried by a harness command.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ReasoningPreference {
    /// Let the harness choose.
    Default,
    /// Request visible reasoning.
    On,
    /// Request hidden reasoning.
    Off,
}

impl ReasoningPreference {
    /// Returns the stable metadata label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::On => "on",
            Self::Off => "off",
        }
    }
}

/// A bounded model, reasoning, and option preference descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelMetadata {
    /// Optional model label.
    pub model_label: Option<String>,
    /// Reasoning visibility preference.
    pub reasoning: ReasoningPreference,
    /// Bounded option catalog labels.
    pub option_labels: Vec<String>,
}

impl ModelMetadata {
    /// Creates bounded model metadata.
    ///
    /// # Errors
    ///
    /// Returns a typed error for an oversized model label, an oversized or
    /// over-count option catalog, or an empty coherence failure.
    pub fn new(
        model_label: Option<&str>,
        reasoning: ReasoningPreference,
        option_labels: &[&str],
    ) -> Result<Self, Error> {
        if let Some(label) = model_label {
            validate_bounded_text(label, MAX_MODEL_LABEL_BYTES)?;
        }
        if option_labels.len() > MAX_OPTION_LABELS {
            return Err(Error::TooManyOptions);
        }
        for label in option_labels {
            validate_bounded_text(label, MAX_OPTION_LABEL_BYTES)?;
        }
        Ok(Self {
            model_label: model_label.map(String::from),
            reasoning,
            option_labels: option_labels.iter().map(|label| (*label).into()).collect(),
        })
    }
}

/// The bounded BLAKE3 identity of one command payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PayloadIdentity {
    /// The raw 32-byte BLAKE3 digest of the payload.
    pub blob_hash: [u8; 32],
    /// The payload byte length.
    pub byte_len: usize,
}

impl PayloadIdentity {
    /// Hashes a bounded command payload.
    ///
    /// # Errors
    ///
    /// Returns a typed error for an empty or oversized payload.
    pub fn from_bytes(payload: &[u8]) -> Result<Self, Error> {
        if payload.is_empty() {
            return Err(Error::EmptyPayload);
        }
        if payload.len() > MAX_PAYLOAD_BYTES {
            return Err(Error::PayloadTooLarge);
        }
        let mut hasher = blake3::Hasher::new();
        hasher.update(PAYLOAD_DOMAIN.as_bytes());
        hasher.update(&[HASH_SEPARATOR]);
        hasher.update(payload);
        Ok(Self {
            blob_hash: *hasher.finalize().as_bytes(),
            byte_len: payload.len(),
        })
    }

    /// Returns the hexadecimal payload identity.
    #[must_use]
    pub fn hex(&self) -> String {
        hex_encode(&self.blob_hash)
    }
}

/// A typed external-agent harness command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessCommand {
    /// The durable operation kind.
    pub kind: CommandKind,
    /// Stable command identifier.
    pub command_id: String,
    /// Target session identifier.
    pub session_id: String,
    /// Owning host identifier.
    pub owner_id: String,
    /// Monotonic per-session sequence.
    pub sequence: u64,
    /// Bounded payload identity.
    pub payload: PayloadIdentity,
    /// Expected outcome class.
    pub expected_outcome: Outcome,
    /// Bounded model, reasoning, and option preferences.
    pub model: Option<ModelMetadata>,
}

impl HarnessCommand {
    /// Creates and validates a typed harness command.
    ///
    /// # Errors
    ///
    /// Returns a typed error for empty or oversized identifiers, a zero
    /// sequence, an unknown outcome, or unbounded model metadata.
    #[allow(clippy::too_many_arguments)] // every durable command field is explicit
    pub fn new(
        kind: CommandKind,
        command_id: &str,
        session_id: &str,
        owner_id: &str,
        sequence: u64,
        payload: PayloadIdentity,
        expected_outcome: Outcome,
        model: Option<ModelMetadata>,
    ) -> Result<Self, Error> {
        validate_identifier(command_id)?;
        validate_identifier(session_id)?;
        validate_identifier(owner_id)?;
        if sequence < MIN_SEQUENCE {
            return Err(Error::SequenceZero);
        }
        let _ = expected_outcome.as_str();
        Ok(Self {
            kind,
            command_id: command_id.into(),
            session_id: session_id.into(),
            owner_id: owner_id.into(),
            sequence,
            payload,
            expected_outcome,
            model,
        })
    }
}

/// One of the recognized harness command outcome classes.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Outcome {
    /// The run was created and is live.
    Created,
    /// The run completed.
    Completed,
    /// The run was steered.
    Steered,
    /// The run was interrupted.
    Interrupted,
    /// The harness requests input.
    InputRequired,
    /// The run was cancelled.
    Cancelled,
    /// The run timed out.
    TimedOut,
    /// The harness service is unavailable.
    Unavailable,
    /// The harness observation was malformed.
    Malformed,
}

impl Outcome {
    /// Returns the stable metadata label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Completed => "completed",
            Self::Steered => "steered",
            Self::Interrupted => "interrupted",
            Self::InputRequired => "input_required",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
            Self::Unavailable => "unavailable",
            Self::Malformed => "malformed",
        }
    }

    /// Returns whether the outcome is a terminal class.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        !matches!(self, Self::Created | Self::InputRequired)
    }
}

/// The durable identity-carrying envelope of one command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandEnvelope {
    /// The durable operation kind.
    pub kind: CommandKind,
    /// Stable command identifier.
    pub command_id: String,
    /// Target session identifier.
    pub session_id: String,
    /// Owning host identifier.
    pub owner_id: String,
    /// Monotonic per-session sequence.
    pub sequence: u64,
    /// Expected outcome class.
    pub expected_outcome: Outcome,
    /// Hexadecimal payload identity.
    pub payload_hash_hex: String,
    /// Payload byte length.
    pub payload_len: usize,
}

impl CommandEnvelope {
    /// Builds an envelope from a command and, when present, its raw payload.
    ///
    /// # Errors
    ///
    /// Returns a typed error for bounded-identity or payload-hash violations.
    pub fn from_command(
        command: &HarnessCommand,
        raw_payload: Option<&[u8]>,
    ) -> Result<Self, Error> {
        let payload_hash_hex;
        let payload_len;
        if let Some(payload) = raw_payload {
            let identity = PayloadIdentity::from_bytes(payload)?;
            if identity.blob_hash != command.payload.blob_hash {
                return Err(Error::PayloadHashMismatch);
            }
            payload_hash_hex = identity.hex();
            payload_len = identity.byte_len;
        } else {
            payload_hash_hex = command.payload.hex();
            payload_len = command.payload.byte_len;
        }
        if command.payload.byte_len != payload_len {
            return Err(Error::IdentityLeak);
        }
        validate_hash(&payload_hash_hex)?;
        Ok(Self {
            kind: command.kind,
            command_id: command.command_id.clone(),
            session_id: command.session_id.clone(),
            owner_id: command.owner_id.clone(),
            sequence: command.sequence,
            expected_outcome: command.expected_outcome,
            payload_hash_hex,
            payload_len,
        })
    }

    /// Derives the stable command identity over envelope fields and the
    /// payload identity. Raw payload bodies never enter identity material.
    #[must_use]
    pub fn identity(&self) -> String {
        digest(COMMAND_IDENTITY_DOMAIN, &self.canonical())
    }

    fn canonical(&self) -> Vec<u8> {
        let mut material = Vec::new();
        material.extend_from_slice(self.kind.as_str().as_bytes());
        material.push(HASH_SEPARATOR);
        material.extend_from_slice(self.command_id.as_bytes());
        material.push(HASH_SEPARATOR);
        material.extend_from_slice(self.session_id.as_bytes());
        material.push(HASH_SEPARATOR);
        material.extend_from_slice(self.owner_id.as_bytes());
        material.push(HASH_SEPARATOR);
        material.extend_from_slice(&self.sequence.to_be_bytes());
        material.push(HASH_SEPARATOR);
        material.extend_from_slice(self.expected_outcome.as_str().as_bytes());
        material.push(HASH_SEPARATOR);
        material.extend_from_slice(self.payload_hash_hex.as_bytes());
        material.push(HASH_SEPARATOR);
        material.extend_from_slice(&self.payload_len.to_be_bytes());
        material
    }
}

/// Validates that an envelope identity is hash-based and matches its payload.
///
/// # Errors
///
/// Returns a typed error when the raw payload is empty, oversized, mismatched,
/// or when the supplied identity candidate embeds raw body material.
pub fn validate_identity_material(
    envelope: &CommandEnvelope,
    raw_payload: &[u8],
    identity_candidate: &str,
) -> Result<(), Error> {
    let identity = PayloadIdentity::from_bytes(raw_payload)?;
    if identity.hex() != envelope.payload_hash_hex {
        return Err(Error::IdentityLeak);
    }
    if identity_candidate != envelope.identity() {
        return Err(Error::IdentityLeak);
    }
    Ok(())
}

/// The target session run that is currently live.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveRun {
    /// Target session identifier.
    pub session_id: String,
    /// Bounded run identifier.
    pub run_id: String,
}

impl ActiveRun {
    /// Creates a validated active run.
    ///
    /// # Errors
    ///
    /// Returns a typed error for empty or oversized identifiers.
    pub fn new(session_id: &str, run_id: &str) -> Result<Self, Error> {
        validate_identifier(session_id)?;
        validate_identifier(run_id)?;
        Ok(Self {
            session_id: session_id.into(),
            run_id: run_id.into(),
        })
    }
}

/// The admission status of the bounded external-process harness service.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HarnessStatus {
    /// The harness service is available.
    Available,
    /// The harness service is unavailable or not started.
    Unavailable,
}

/// The admitted plane action for one command evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlaneAction {
    /// Queue the command without executing it.
    Enqueue,
    /// Execute the command at the harness.
    Execute,
    /// Dispatch the command as the next turn.
    DispatchNextTurn,
    /// Skip the command because it is already processed.
    SkipProcessed,
    /// Keep the command queued because the harness is unavailable.
    FailUnavailable,
}

/// The pure durable command-plane state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlaneState {
    /// The identity of the host that owns the target session.
    pub owner_id: String,
    /// The currently live run, when one exists.
    pub active_run: Option<ActiveRun>,
    /// Processed command identities, oldest first, bounded by cap.
    pub processed: Vec<String>,
    /// Queued command identities that are not transcript content.
    pub queue: Vec<String>,
    /// The retained processed-command ledger cap.
    pub processed_cap: usize,
    /// The retained queued-command cap.
    pub queue_cap: usize,
}

impl PlaneState {
    /// Creates a validated command-plane state.
    ///
    /// # Errors
    ///
    /// Returns a typed error for an empty or oversized owner, or a ledger cap
    /// outside the accepted bounds.
    pub fn new(owner_id: &str) -> Result<Self, Error> {
        Self::with_caps(owner_id, MAX_PROCESSED_ENTRIES, MAX_QUEUE_ENTRIES)
    }

    /// Creates a command-plane state with explicit ledger caps.
    ///
    /// # Errors
    ///
    /// Returns a typed error for an empty or oversized owner, or caps outside
    /// the accepted bounds.
    pub fn with_caps(
        owner_id: &str,
        processed_cap: usize,
        queue_cap: usize,
    ) -> Result<Self, Error> {
        validate_identifier(owner_id)?;
        validate_bounded_count(processed_cap, MAX_PROCESSED_ENTRIES, Error::MissingLimit)?;
        validate_bounded_count(queue_cap, MAX_QUEUE_ENTRIES, Error::MissingLimit)?;
        Ok(Self {
            owner_id: owner_id.into(),
            active_run: None,
            processed: Vec::new(),
            queue: Vec::new(),
            processed_cap,
            queue_cap,
        })
    }

    /// Returns whether the command identity is already processed.
    #[must_use]
    pub fn is_processed(&self, identity: &str) -> bool {
        self.processed.iter().any(|entry| entry == identity)
    }

    /// Returns the retained processed-command count.
    #[must_use]
    pub fn processed_len(&self) -> usize {
        self.processed.len()
    }

    /// Returns the retained queued-command count.
    #[must_use]
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }
}

/// The admission and execution decision for one command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlaneDecision {
    /// The admitted plane action.
    pub action: PlaneAction,
    /// The stable command identity.
    pub identity: String,
}

/// Evaluates one durable command against the plane and harness status.
///
/// Execution authority is owner-gated, processed commands are skipped, and
/// steer commands with no live active run dispatch as the next turn. Commands
/// for an unavailable harness stay queued and are never dropped.
#[must_use]
pub fn evaluate_command(
    state: &PlaneState,
    command: &HarnessCommand,
    envelope: &CommandEnvelope,
    harness_status: HarnessStatus,
) -> PlaneDecision {
    let identity = envelope.identity();
    if state.is_processed(&identity) {
        return PlaneDecision {
            action: PlaneAction::SkipProcessed,
            identity,
        };
    }
    if state.owner_id != command.owner_id {
        return PlaneDecision {
            action: PlaneAction::Enqueue,
            identity,
        };
    }
    if harness_status == HarnessStatus::Unavailable {
        return PlaneDecision {
            action: PlaneAction::FailUnavailable,
            identity,
        };
    }
    let action = match command.kind {
        CommandKind::Send | CommandKind::Interrupt => PlaneAction::Execute,
        CommandKind::Steer | CommandKind::RespondInput => {
            if state.active_run.is_some() {
                PlaneAction::Execute
            } else {
                PlaneAction::DispatchNextTurn
            }
        }
    };
    PlaneDecision { action, identity }
}

/// Marks one command as processed and returns the marker plus the new state.
///
/// The host MUST call this before executing the command and MUST commit the
/// returned state before any harness effect. This is the
/// mark-processed-before-execute invariant.
///
/// # Errors
///
/// Returns a typed error when the retained ledger would exceed its cap.
pub fn begin_execution(
    mut state: PlaneState,
    envelope: &CommandEnvelope,
) -> Result<(PlaneState, ProcessedMarker), Error> {
    let identity = envelope.identity();
    if state.is_processed(&identity) {
        return Ok((state, ProcessedMarker { identity }));
    }
    if state.processed.len() >= state.processed_cap {
        return Err(Error::ProcessedLedgerOverflow);
    }
    state.processed.push(identity.clone());
    Ok((state, ProcessedMarker { identity }))
}

/// A processed-command marker issued before host execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessedMarker {
    /// The processed command identity.
    pub identity: String,
}

/// Records a harness outcome against a processed marker.
///
/// # Errors
///
/// Returns a typed error when the marker is absent from the ledger or when
/// receipt material is unsafe.
pub fn complete_execution(
    state: &PlaneState,
    marker: &ProcessedMarker,
    outcome: &BoundedProcessOutcome,
    harness_family: &str,
) -> Result<HarnessReceipt, Error> {
    if !state.is_processed(&marker.identity) {
        return Err(Error::StaleMarker);
    }
    validate_bounded_text(harness_family, MAX_LABEL_BYTES)?;
    HarnessReceipt::new(marker, outcome, harness_family)
}

/// Queues one command identity without transcript admission.
///
/// # Errors
///
/// Returns a typed error when the queue exceeds its cap.
pub fn enqueue_command(
    mut state: PlaneState,
    envelope: &CommandEnvelope,
) -> Result<(PlaneState, PlaneAction), Error> {
    let identity = envelope.identity();
    if state.queue.iter().any(|entry| entry == &identity) {
        return Ok((state, PlaneAction::Enqueue));
    }
    if state.queue.len() >= state.queue_cap {
        return Err(Error::QueueOverflow);
    }
    state.queue.push(identity);
    Ok((state, PlaneAction::Enqueue))
}

/// The environment admission mode for one bounded process.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnvironmentMode {
    /// Inherit the caller environment.
    Inherit,
    /// Start from an empty environment.
    Clear,
}

/// An explicit bounded external-process admission request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedProcessRequest {
    /// The program path or command name.
    pub program: String,
    /// Argument list.
    pub args: Vec<String>,
    /// Optional working directory.
    pub current_dir: Option<String>,
    /// Environment admission mode.
    pub environment_mode: EnvironmentMode,
    /// Explicit environment entries when the mode is clear.
    pub environment: Vec<(String, String)>,
    /// Standard-input byte bound.
    pub stdin_max_bytes: usize,
    /// Standard-output byte bound.
    pub stdout_max_bytes: usize,
    /// Standard-error byte bound.
    pub stderr_max_bytes: usize,
    /// Process window in milliseconds.
    pub timeout_ms: u64,
    /// Teardown window in milliseconds.
    pub teardown_timeout_ms: u64,
    /// Poll interval in milliseconds.
    pub poll_interval_ms: u64,
}

impl BoundedProcessRequest {
    /// Creates and validates an explicit bounded process request.
    ///
    /// Every execution bound must be explicit. No implicit default limits are
    /// applied when a field is absent.
    ///
    /// # Errors
    ///
    /// Returns a typed error when the program is empty or oversized, an
    /// argument or environment entry exceeds its bound, or any required limit
    /// is missing or outside its accepted range.
    #[allow(clippy::too_many_arguments)] // every execution bound is explicit
    pub fn new(
        program: &str,
        args: &[&str],
        current_dir: Option<&str>,
        environment_mode: EnvironmentMode,
        environment: &[(&str, &str)],
        stdin_max_bytes: usize,
        stdout_max_bytes: usize,
        stderr_max_bytes: usize,
        timeout_ms: u64,
        teardown_timeout_ms: u64,
        poll_interval_ms: u64,
    ) -> Result<Self, Error> {
        if program.is_empty() {
            return Err(Error::EmptyIdentifier);
        }
        validate_bounded_text(program, MAX_PROGRAM_BYTES)?;
        if args.len() > MAX_ARGS {
            return Err(Error::TooManyOptions);
        }
        for arg in args {
            validate_bounded_text(arg, MAX_ARG_BYTES)?;
        }
        if let Some(dir) = current_dir {
            validate_bounded_text(dir, MAX_IDENTIFIER_BYTES)?;
        }
        if environment.len() > MAX_ENV_ENTRIES {
            return Err(Error::TooManyOptions);
        }
        for (key, value) in environment {
            validate_bounded_text(key, MAX_ENV_BYTES)?;
            validate_bounded_text(value, MAX_ENV_BYTES)?;
        }
        validate_bounded_count(stdin_max_bytes, MAX_STDIN_BYTES, Error::MissingLimit)?;
        validate_bounded_count(stdout_max_bytes, MAX_STREAM_BYTES, Error::MissingLimit)?;
        validate_bounded_count(stderr_max_bytes, MAX_STREAM_BYTES, Error::MissingLimit)?;
        validate_process_window(timeout_ms)?;
        validate_process_window(teardown_timeout_ms)?;
        validate_process_window(poll_interval_ms)?;
        Ok(Self {
            program: program.into(),
            args: args.iter().map(|arg| (*arg).into()).collect(),
            current_dir: current_dir.map(String::from),
            environment_mode,
            environment: environment
                .iter()
                .map(|(key, value)| ((*key).into(), (*value).into()))
                .collect(),
            stdin_max_bytes,
            stdout_max_bytes,
            stderr_max_bytes,
            timeout_ms,
            teardown_timeout_ms,
            poll_interval_ms,
        })
    }
}

fn validate_process_window(ms: u64) -> Result<(), Error> {
    if ms < MIN_PROCESS_WINDOW_MS {
        return Err(Error::MissingLimit);
    }
    if ms > MAX_PROCESS_WINDOW_MS {
        return Err(Error::LimitTooLarge);
    }
    Ok(())
}

/// A bounded observation returned by the injected process service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundedProcessOutcome {
    /// The process completed within bounds.
    Completed {
        /// The observed exit class.
        exit_code: i32,
        /// The bounded standard-output byte count retained.
        stdout_bytes: usize,
        /// The bounded standard-error byte count retained.
        stderr_bytes: usize,
        /// Whether the observation reports a deadline hit.
        timed_out: bool,
    },
    /// The process exceeded its deadline.
    TimedOut,
    /// The process was cancelled before completion.
    Cancelled,
    /// No process service could start the program.
    Unavailable,
    /// The process observation was malformed.
    MalformedObservation,
}

impl BoundedProcessOutcome {
    /// Returns the metadata outcome class label.
    #[must_use]
    pub fn outcome_class(&self) -> Outcome {
        match self {
            Self::Completed { .. } => Outcome::Completed,
            Self::TimedOut => Outcome::TimedOut,
            Self::Cancelled => Outcome::Cancelled,
            Self::Unavailable => Outcome::Unavailable,
            Self::MalformedObservation => Outcome::Malformed,
        }
    }
}

/// The readiness of the injected bounded process service.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceStatus {
    /// The service is injected and available.
    Ready,
    /// The service is absent or denied.
    Unavailable,
}

/// The pure dispatch admission for one process request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DispatchDecision {
    /// Dispatch the validated request to the host transport.
    Dispatch(BoundedProcessRequest),
    /// Fail closed with a typed unavailable outcome and no fallback.
    FailClosed(BoundedProcessOutcome),
}

/// Plans harness dispatch without touching a process, transport, or clock.
#[must_use]
pub fn plan_harness_dispatch(
    request: BoundedProcessRequest,
    service_status: ServiceStatus,
) -> DispatchDecision {
    match service_status {
        ServiceStatus::Ready => DispatchDecision::Dispatch(request),
        ServiceStatus::Unavailable => {
            DispatchDecision::FailClosed(BoundedProcessOutcome::Unavailable)
        }
    }
}

/// An implementation-free port for bounded external-process execution.
///
/// Hosts implement this trait over a pinned bounded-exec surface. Reusable
/// harness code never spawns processes or discovers executables.
pub trait BoundedAgentProcessService {
    /// Runs one bounded process request.
    ///
    /// # Errors
    ///
    /// Returns a typed unavailable signal when the service cannot run the
    /// program at all.
    fn run(
        &self,
        request: &BoundedProcessRequest,
    ) -> Result<BoundedProcessOutcome, ServiceUnavailable>;
}

/// The typed unavailable-process-service signal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServiceUnavailable;

/// A metadata-only receipt for one command-plane admission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlaneReceipt {
    /// Stable command identifier.
    pub command_id: String,
    /// Target session identifier.
    pub session_id: String,
    /// Owning host identifier.
    pub owner_id: String,
    /// Durable operation kind.
    pub kind: CommandKind,
    /// Monotonic per-session sequence.
    pub sequence: u64,
    /// Hexadecimal command identity.
    pub command_hash_hex: String,
    /// Hexadecimal payload identity.
    pub payload_hash_hex: String,
    /// Admitted plane action.
    pub action: PlaneAction,
    /// Bounded processed-command count.
    pub processed_count: usize,
    /// Bounded queued-command count.
    pub queue_count: usize,
}

impl PlaneReceipt {
    /// Creates a validated metadata-only plane receipt.
    ///
    /// # Errors
    ///
    /// Returns a typed error for empty or oversized identifiers or malformed
    /// hexadecimal identities.
    pub fn new(
        command: &HarnessCommand,
        envelope: &CommandEnvelope,
        action: PlaneAction,
        state: &PlaneState,
    ) -> Result<Self, Error> {
        validate_identifier(&command.command_id)?;
        validate_identifier(&command.session_id)?;
        validate_identifier(&command.owner_id)?;
        validate_hash(&envelope.identity())?;
        validate_hash(&envelope.payload_hash_hex)?;
        Ok(Self {
            command_id: command.command_id.clone(),
            session_id: command.session_id.clone(),
            owner_id: command.owner_id.clone(),
            kind: command.kind,
            sequence: command.sequence,
            command_hash_hex: envelope.identity(),
            payload_hash_hex: envelope.payload_hash_hex.clone(),
            action,
            processed_count: state.processed_len(),
            queue_count: state.queue_len(),
        })
    }

    /// Derives the deterministic receipt identity.
    #[must_use]
    pub fn identity(&self) -> String {
        digest(
            RECEIPT_DOMAIN,
            format!(
                "{}|{}|{}|{}|{}|{}|{}",
                self.command_id,
                self.session_id,
                self.owner_id,
                self.kind.as_str(),
                self.sequence,
                self.command_hash_hex,
                self.payload_hash_hex
            )
            .as_bytes(),
        )
    }
}

/// A metadata-only receipt for one harness execution observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessReceipt {
    /// Processed command identity hex.
    pub command_hash_hex: String,
    /// Harness family label.
    pub harness_family: String,
    /// Metadata outcome class.
    pub outcome_class: Outcome,
    /// Bounded standard-output byte count.
    pub stdout_bytes: usize,
    /// Bounded standard-error byte count.
    pub stderr_bytes: usize,
    /// Whether the observation reports a deadline hit.
    pub timed_out: bool,
}

impl HarnessReceipt {
    /// Creates a validated metadata-only harness receipt.
    ///
    /// # Errors
    ///
    /// Returns a typed error for a malformed command identity or an oversized
    /// family label.
    pub fn new(
        marker: &ProcessedMarker,
        outcome: &BoundedProcessOutcome,
        harness_family: &str,
    ) -> Result<Self, Error> {
        validate_hash(&marker.identity)?;
        validate_bounded_text(harness_family, MAX_LABEL_BYTES)?;
        let (stdout_bytes, stderr_bytes, timed_out) = match outcome {
            BoundedProcessOutcome::Completed {
                stdout_bytes,
                stderr_bytes,
                timed_out,
                ..
            } => (*stdout_bytes, *stderr_bytes, *timed_out),
            _ => (0, 0, false),
        };
        Ok(Self {
            command_hash_hex: marker.identity.clone(),
            harness_family: harness_family.into(),
            outcome_class: outcome.outcome_class(),
            stdout_bytes,
            stderr_bytes,
            timed_out,
        })
    }

    /// Derives the deterministic receipt identity.
    #[must_use]
    pub fn identity(&self) -> String {
        digest(
            RECEIPT_DOMAIN,
            format!(
                "{}|{}|{}|{}|{}|{}",
                self.command_hash_hex,
                self.harness_family,
                self.outcome_class.as_str(),
                self.stdout_bytes,
                self.stderr_bytes,
                self.timed_out
            )
            .as_bytes(),
        )
    }
}

/// A class of raw host material that receipts must never embed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RawMaterialClass {
    /// Raw prompt text.
    RawPrompt,
    /// Raw harness JSONL lines.
    HarnessJsonl,
    /// Provider payload bodies.
    ProviderPayload,
    /// Tool call input or output bodies.
    ToolBody,
    /// Model output bodies.
    ModelOutput,
    /// Token, key, or credential bodies.
    Credential,
    /// Full host path dumps.
    FullPath,
    /// Secret summaries.
    SecretSummary,
}

impl RawMaterialClass {
    /// Returns every recognized raw material class.
    #[must_use]
    pub const fn all() -> [Self; 8] {
        [
            Self::RawPrompt,
            Self::HarnessJsonl,
            Self::ProviderPayload,
            Self::ToolBody,
            Self::ModelOutput,
            Self::Credential,
            Self::FullPath,
            Self::SecretSummary,
        ]
    }

    /// Returns the stable metadata label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawPrompt => "raw_prompt",
            Self::HarnessJsonl => "harness_jsonl",
            Self::ProviderPayload => "provider_payload",
            Self::ToolBody => "tool_body",
            Self::ModelOutput => "model_output",
            Self::Credential => "credential",
            Self::FullPath => "full_path",
            Self::SecretSummary => "secret_summary",
        }
    }
}

/// A receipt candidate that reports which raw classes a host tried to keep.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReceiptCandidate {
    /// Raw material classes that would leak into durable evidence.
    pub raw_attachments: Vec<RawMaterialClass>,
}

impl ReceiptCandidate {
    /// Creates an empty metadata-only receipt candidate.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            raw_attachments: Vec::new(),
        }
    }

    /// Creates a candidate that attempts to attach raw material.
    #[must_use]
    pub fn with_raw(raw_attachments: Vec<RawMaterialClass>) -> Self {
        Self { raw_attachments }
    }
}

/// Enforces the metadata-only receipt discipline.
///
/// # Errors
///
/// Returns the first raw material leak class when the candidate carries any
/// raw attachment.
pub fn validate_metadata_only(candidate: &ReceiptCandidate) -> Result<(), Error> {
    if !candidate.raw_attachments.is_empty() {
        return Err(Error::RawMaterialLeak);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMMAND: &str = "cmd-0001";
    const SESSION: &str = "session-001";
    const OWNER: &str = "device-a";
    const PAYLOAD: &[u8] = b"implement the zeron harness contract";

    fn payload_identity() -> PayloadIdentity {
        PayloadIdentity::from_bytes(PAYLOAD).expect("payload hashes")
    }

    fn command(kind: CommandKind, sequence: u64) -> HarnessCommand {
        HarnessCommand::new(
            kind,
            COMMAND,
            SESSION,
            OWNER,
            sequence,
            payload_identity(),
            Outcome::Completed,
            None,
        )
        .expect("command validates")
    }

    fn envelope(command: &HarnessCommand) -> CommandEnvelope {
        CommandEnvelope::from_command(command, Some(PAYLOAD)).expect("envelope validates")
    }

    #[test]
    fn payload_identity_is_bounded_and_deterministic() {
        let first = PayloadIdentity::from_bytes(PAYLOAD).expect("hash succeeds");
        let second = PayloadIdentity::from_bytes(PAYLOAD).expect("hash succeeds");
        assert_eq!(first, second);
        assert_eq!(first.byte_len, PAYLOAD.len());
        assert_eq!(first.hex().len(), BLAKE3_HEX_CHARS);
    }

    #[test]
    fn payload_identity_rejects_empty_and_oversized() {
        assert_eq!(PayloadIdentity::from_bytes(&[]), Err(Error::EmptyPayload));
        let oversized = alloc::vec![0_u8; MAX_PAYLOAD_BYTES + 1];
        assert_eq!(
            PayloadIdentity::from_bytes(&oversized),
            Err(Error::PayloadTooLarge)
        );
    }

    #[test]
    fn command_identity_is_hash_based() {
        let command = command(CommandKind::Send, 1);
        let envelope = envelope(&command);
        let identity = envelope.identity();
        assert_eq!(identity.len(), BLAKE3_HEX_CHARS);
        validate_identity_material(&envelope, PAYLOAD, &identity).expect("identity is hash-based");
    }

    #[test]
    fn raw_body_in_identity_is_rejected() {
        let command = command(CommandKind::Send, 1);
        let envelope = envelope(&command);
        let wrong_payload: &[u8] = b"a different raw body";
        assert_eq!(
            validate_identity_material(&envelope, wrong_payload, &envelope.identity()),
            Err(Error::IdentityLeak)
        );
        assert_eq!(
            validate_identity_material(&envelope, PAYLOAD, "raw-body-as-identity"),
            Err(Error::IdentityLeak)
        );
    }

    #[test]
    fn envelope_rejects_payload_hash_mismatch() {
        let command = command(CommandKind::Send, 1);
        assert_eq!(
            CommandEnvelope::from_command(&command, Some(b"other body")),
            Err(Error::PayloadHashMismatch)
        );
    }

    #[test]
    fn command_validation_rejects_bad_inputs() {
        assert_eq!(
            HarnessCommand::new(
                CommandKind::Send,
                "",
                SESSION,
                OWNER,
                1,
                payload_identity(),
                Outcome::Completed,
                None,
            ),
            Err(Error::EmptyIdentifier)
        );
        assert_eq!(
            HarnessCommand::new(
                CommandKind::Send,
                COMMAND,
                SESSION,
                OWNER,
                0,
                payload_identity(),
                Outcome::Completed,
                None,
            ),
            Err(Error::SequenceZero)
        );
    }

    #[test]
    fn model_metadata_enforces_bounds() {
        assert!(
            ModelMetadata::new(Some("claude"), ReasoningPreference::On, &["plan", "review"])
                .is_ok()
        );
        let long_label =
            alloc::string::String::from_utf8(alloc::vec![b'x'; MAX_MODEL_LABEL_BYTES + 1])
                .expect("utf8");
        assert_eq!(
            ModelMetadata::new(Some(&long_label), ReasoningPreference::On, &[]),
            Err(Error::ValueTooLong)
        );
    }

    #[test]
    fn owner_gated_execution_and_non_owner_queue() {
        let state = PlaneState::new(OWNER).expect("plane initializes");
        let other = command(CommandKind::Send, 1);
        let other_envelope = envelope(&other);
        let other_command = HarnessCommand::new(
            CommandKind::Send,
            COMMAND,
            SESSION,
            "device-b",
            2,
            payload_identity(),
            Outcome::Completed,
            None,
        )
        .expect("other command");
        let decision = evaluate_command(
            &state,
            &other_command,
            &other_envelope,
            HarnessStatus::Available,
        );
        assert_eq!(decision.action, PlaneAction::Enqueue);
    }

    #[test]
    fn owner_executes_send_and_interrupt() {
        let state = PlaneState::new(OWNER).expect("plane initializes");
        for kind in [CommandKind::Send, CommandKind::Interrupt] {
            let command = command(kind, 1);
            let envelope = envelope(&command);
            let decision = evaluate_command(&state, &command, &envelope, HarnessStatus::Available);
            assert_eq!(decision.action, PlaneAction::Execute);
        }
    }

    #[test]
    fn steer_with_no_active_run_dispatches_next_turn() {
        let state = PlaneState::new(OWNER).expect("plane initializes");
        let command = command(CommandKind::Steer, 1);
        let envelope = envelope(&command);
        let decision = evaluate_command(&state, &command, &envelope, HarnessStatus::Available);
        assert_eq!(decision.action, PlaneAction::DispatchNextTurn);
    }

    #[test]
    fn steer_with_active_run_executes() {
        let mut state = PlaneState::new(OWNER).expect("plane initializes");
        state.active_run = Some(ActiveRun::new(SESSION, "run-1").expect("run"));
        let command = command(CommandKind::Steer, 1);
        let envelope = envelope(&command);
        let decision = evaluate_command(&state, &command, &envelope, HarnessStatus::Available);
        assert_eq!(decision.action, PlaneAction::Execute);
    }

    #[test]
    fn offline_harness_keeps_commands_queued() {
        let state = PlaneState::new(OWNER).expect("plane initializes");
        let command = command(CommandKind::Send, 1);
        let envelope = envelope(&command);
        let decision = evaluate_command(&state, &command, &envelope, HarnessStatus::Unavailable);
        assert_eq!(decision.action, PlaneAction::FailUnavailable);
    }

    #[test]
    fn processed_commands_are_skipped() {
        let state = PlaneState::new(OWNER).expect("plane initializes");
        let command = command(CommandKind::Send, 1);
        let envelope = envelope(&command);
        let (state, marker) = begin_execution(state, &envelope).expect("marks before execute");
        assert!(state.is_processed(&marker.identity));
        let decision = evaluate_command(&state, &command, &envelope, HarnessStatus::Available);
        assert_eq!(decision.action, PlaneAction::SkipProcessed);
    }

    #[test]
    fn begin_execution_marks_before_execute_and_completes() {
        let state = PlaneState::new(OWNER).expect("plane initializes");
        let command = command(CommandKind::Send, 1);
        let envelope = envelope(&command);
        let (state, marker) = begin_execution(state, &envelope).expect("marks");
        let receipt = complete_execution(
            &state,
            &marker,
            &BoundedProcessOutcome::Completed {
                exit_code: 0,
                stdout_bytes: 12,
                stderr_bytes: 0,
                timed_out: false,
            },
            HARNESS_FAMILY,
        )
        .expect("records outcome");
        assert_eq!(receipt.outcome_class, Outcome::Completed);
        assert_eq!(receipt.harness_family, HARNESS_FAMILY);
        assert_eq!(state.processed_len(), 1);
    }

    #[test]
    fn stale_marker_is_rejected() {
        let state = PlaneState::new(OWNER).expect("plane initializes");
        let marker = ProcessedMarker {
            identity: alloc::vec![b'0'; BLAKE3_HEX_CHARS]
                .into_iter()
                .map(char::from)
                .collect::<alloc::string::String>(),
        };
        assert_eq!(
            complete_execution(
                &state,
                &marker,
                &BoundedProcessOutcome::Unavailable,
                HARNESS_FAMILY
            ),
            Err(Error::StaleMarker)
        );
    }

    #[test]
    fn queue_respects_cap() {
        let state = PlaneState::with_caps(OWNER, 4, 2).expect("plane initializes");
        let first = command(CommandKind::Send, 1);
        let second = command(CommandKind::Send, 2);
        let third = command(CommandKind::Send, 3);
        let (state, _) = enqueue_command(state.clone(), &envelope(&first)).expect("first enqueues");
        let (state, _) =
            enqueue_command(state.clone(), &envelope(&second)).expect("second enqueues");
        assert_eq!(state.queue_len(), 2);
        assert_eq!(
            enqueue_command(state.clone(), &envelope(&third)),
            Err(Error::QueueOverflow)
        );
        assert_eq!(state.queue_len(), 2);
    }

    #[test]
    fn process_request_requires_explicit_limits() {
        let request = BoundedProcessRequest::new(
            "/bin/true",
            &[],
            None,
            EnvironmentMode::Clear,
            &[],
            0,
            4_096,
            4_096,
            5_000,
            500,
            10,
        );
        assert_eq!(request, Err(Error::MissingLimit));
    }

    #[test]
    fn process_request_rejects_oversized_limits() {
        let request = BoundedProcessRequest::new(
            "/bin/true",
            &[],
            None,
            EnvironmentMode::Clear,
            &[],
            128,
            MAX_STREAM_BYTES + 1,
            4_096,
            5_000,
            500,
            10,
        );
        assert_eq!(request, Err(Error::LimitTooLarge));
    }

    #[test]
    fn process_request_accepts_explicit_bounds() {
        let request = BoundedProcessRequest::new(
            "/bin/printf",
            &["hello"],
            None,
            EnvironmentMode::Clear,
            &[],
            128,
            4_096,
            4_096,
            5_000,
            500,
            10,
        );
        assert!(request.is_ok());
    }

    #[test]
    fn dispatch_fails_closed_without_service() {
        let request = BoundedProcessRequest::new(
            "/bin/true",
            &[],
            None,
            EnvironmentMode::Clear,
            &[],
            128,
            4_096,
            4_096,
            5_000,
            500,
            10,
        )
        .expect("request");
        assert_eq!(
            plan_harness_dispatch(request.clone(), ServiceStatus::Unavailable),
            DispatchDecision::FailClosed(BoundedProcessOutcome::Unavailable)
        );
        assert!(matches!(
            plan_harness_dispatch(request, ServiceStatus::Ready),
            DispatchDecision::Dispatch(_)
        ));
    }

    #[test]
    fn plane_receipt_is_metadata_only() {
        let state = PlaneState::new(OWNER).expect("plane initializes");
        let command = command(CommandKind::Send, 1);
        let envelope = envelope(&command);
        let receipt = PlaneReceipt::new(&command, &envelope, PlaneAction::Execute, &state)
            .expect("receipt builds");
        assert_eq!(receipt.command_hash_hex.len(), BLAKE3_HEX_CHARS);
        assert_eq!(receipt.identity().len(), BLAKE3_HEX_CHARS);
        assert_eq!(receipt.kind, CommandKind::Send);
    }

    #[test]
    fn harness_receipt_is_bounded_and_deterministic() {
        let state = PlaneState::new(OWNER).expect("plane initializes");
        let command = command(CommandKind::Send, 1);
        let envelope = envelope(&command);
        let (state, marker) = begin_execution(state, &envelope).expect("marker");
        let receipt = complete_execution(
            &state,
            &marker,
            &BoundedProcessOutcome::TimedOut,
            HARNESS_FAMILY,
        )
        .expect("receipt");
        assert_eq!(receipt.outcome_class, Outcome::TimedOut);
        assert_eq!(receipt.identity().len(), BLAKE3_HEX_CHARS);
    }

    #[test]
    fn receipt_candidate_rejects_raw_material() {
        assert!(validate_metadata_only(&ReceiptCandidate::empty()).is_ok());
        for class in RawMaterialClass::all() {
            let candidate = ReceiptCandidate::with_raw(alloc::vec![class]);
            assert_eq!(
                validate_metadata_only(&candidate),
                Err(Error::RawMaterialLeak)
            );
        }
    }

    #[test]
    fn outcome_classes_are_stable() {
        assert_eq!(Outcome::Unavailable.as_str(), "unavailable");
        assert!(Outcome::Completed.is_terminal());
        assert!(!Outcome::InputRequired.is_terminal());
        for outcome in [
            Outcome::Created,
            Outcome::Completed,
            Outcome::Steered,
            Outcome::Interrupted,
            Outcome::InputRequired,
            Outcome::Cancelled,
            Outcome::TimedOut,
            Outcome::Unavailable,
            Outcome::Malformed,
        ] {
            assert!(!outcome.as_str().is_empty());
        }
    }

    #[test]
    fn command_kind_labels_are_stable() {
        assert_eq!(CommandKind::from_label("steer"), Some(CommandKind::Steer));
        assert_eq!(CommandKind::from_label("nope"), None);
        for kind in [
            CommandKind::Send,
            CommandKind::Steer,
            CommandKind::Interrupt,
            CommandKind::RespondInput,
        ] {
            assert_eq!(CommandKind::from_label(kind.as_str()), Some(kind));
        }
    }

    #[test]
    fn hex_helpers_round_trip() {
        let identity = payload_identity();
        let hex = identity.hex();
        let decoded = hex_decode(&hex).expect("decodes");
        assert_eq!(decoded, identity.blob_hash);
        assert!(hex_decode("not-hex-that-fits").is_none());
    }
}
