//! Bounded opaque worker-event payload contracts.

const PRIVATE_EVENT_IDENTITY_DOMAIN: &str = "animus.local-daemon.private-event.v1";
const IDENTITY_SEPARATOR: &[u8] = b"\0";

/// Maximum bytes in one private worker event.
pub const MAX_PRIVATE_EVENT_BYTES: u32 = 65_536;

/// Exact facts that bind one private payload to its ordered worker event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrivateEventIdentityFacts<'a> {
    /// Root session identity.
    pub root_id: &'a str,
    /// Monotonic worker event sequence.
    pub sequence: u64,
    /// Safe private event class.
    pub class: &'a str,
}

/// Bounded private event payload routed without supervisor parsing or retention.
// r[impl agent_harness_runtime.live_terminal_agent_host.private_events]
#[derive(Clone, Eq, PartialEq)]
pub struct WorkerPrivateEvent {
    root_id: String,
    sequence: u64,
    class: String,
    payload_identity: String,
    private_payload: Vec<u8>,
}

impl WorkerPrivateEvent {
    /// Constructs one bounded event and binds its payload with BLAKE3.
    ///
    /// # Errors
    ///
    /// Returns [`PrivateEventError::Bound`] for invalid facts or payload size.
    pub fn new(
        facts: PrivateEventIdentityFacts<'_>,
        private_payload: Vec<u8>,
    ) -> Result<Self, PrivateEventError> {
        let payload_identity = private_event_payload_identity(facts, private_payload.as_slice())?;
        Ok(Self {
            root_id: String::from(facts.root_id),
            sequence: facts.sequence,
            class: String::from(facts.class),
            payload_identity,
            private_payload,
        })
    }

    /// Returns the root session identity.
    #[must_use]
    pub fn root_id(&self) -> &str {
        self.root_id.as_str()
    }

    /// Returns the monotonic worker event sequence.
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the safe private event class.
    #[must_use]
    pub fn class(&self) -> &str {
        self.class.as_str()
    }

    /// Returns the BLAKE3 identity of the exact payload and event facts.
    #[must_use]
    pub fn payload_identity(&self) -> &str {
        self.payload_identity.as_str()
    }

    /// Returns the private payload bytes.
    #[must_use]
    pub fn private_payload(&self) -> &[u8] {
        self.private_payload.as_slice()
    }

    /// Consumes the event and returns its wire parts.
    #[must_use]
    pub fn into_wire_parts(self) -> (u64, String, String, Vec<u8>) {
        (
            self.sequence,
            self.class,
            self.payload_identity,
            self.private_payload,
        )
    }

    pub(crate) fn validate(&self, maximum_private_bytes: u32) -> Result<(), PrivateEventError> {
        validate_private_event_payload(
            PrivateEventIdentityFacts {
                root_id: self.root_id.as_str(),
                sequence: self.sequence,
                class: self.class.as_str(),
            },
            self.payload_identity.as_str(),
            self.private_payload.as_slice(),
            maximum_private_bytes,
        )
    }
}

impl core::fmt::Debug for WorkerPrivateEvent {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("WorkerPrivateEvent")
            .field("root_id", &self.root_id)
            .field("sequence", &self.sequence)
            .field("class", &self.class)
            .field("payload_identity", &self.payload_identity)
            .field("private_payload_bytes", &self.private_payload.len())
            .finish()
    }
}

/// Private event validation error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrivateEventError {
    /// Event facts or payload size exceed the closed contract.
    Bound,
    /// The supplied payload identity does not bind the exact event.
    Identity,
}

impl core::fmt::Display for PrivateEventError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "private worker event rejected: {self:?}")
    }
}

impl std::error::Error for PrivateEventError {}

/// Computes the BLAKE3 identity for exact private event facts and payload bytes.
///
/// # Errors
///
/// Returns [`PrivateEventError::Bound`] for invalid facts or payload size.
pub fn private_event_payload_identity(
    facts: PrivateEventIdentityFacts<'_>,
    private_payload: &[u8],
) -> Result<String, PrivateEventError> {
    validate_facts(facts)?;
    validate_payload_size(private_payload, MAX_PRIVATE_EVENT_BYTES)?;
    debug_assert!(facts.sequence > 0, "validated event sequence is positive");
    debug_assert!(
        !private_payload.is_empty(),
        "validated private event payload is not empty"
    );
    let sequence = facts.sequence.to_le_bytes();
    let payload_len = u64::try_from(private_payload.len())
        .map_err(|_error| PrivateEventError::Bound)?
        .to_le_bytes();
    let mut hasher = blake3::Hasher::new();
    for part in [
        PRIVATE_EVENT_IDENTITY_DOMAIN.as_bytes(),
        facts.root_id.as_bytes(),
        sequence.as_slice(),
        facts.class.as_bytes(),
        payload_len.as_slice(),
        private_payload,
    ] {
        hasher.update(part);
        hasher.update(IDENTITY_SEPARATOR);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

/// Validates one relayed payload without parsing its private bytes.
///
/// # Errors
///
/// Returns [`PrivateEventError`] for invalid facts, bounds, or identity drift.
pub fn validate_private_event_payload(
    facts: PrivateEventIdentityFacts<'_>,
    payload_identity: &str,
    private_payload: &[u8],
    maximum_private_bytes: u32,
) -> Result<(), PrivateEventError> {
    validate_facts(facts)?;
    validate_payload_size(
        private_payload,
        maximum_private_bytes.min(MAX_PRIVATE_EVENT_BYTES),
    )?;
    let expected = private_event_payload_identity(facts, private_payload)?;
    if expected != payload_identity {
        return Err(PrivateEventError::Identity);
    }
    Ok(())
}

fn validate_facts(facts: PrivateEventIdentityFacts<'_>) -> Result<(), PrivateEventError> {
    if facts.sequence == 0 {
        return Err(PrivateEventError::Bound);
    }
    validate_label(facts.root_id)?;
    validate_label(facts.class)
}

fn validate_payload_size(
    private_payload: &[u8],
    maximum_private_bytes: u32,
) -> Result<(), PrivateEventError> {
    let maximum =
        usize::try_from(maximum_private_bytes).map_err(|_error| PrivateEventError::Bound)?;
    if private_payload.is_empty() || private_payload.len() > maximum {
        return Err(PrivateEventError::Bound);
    }
    Ok(())
}

fn validate_label(value: &str) -> Result<(), PrivateEventError> {
    const MAX_LABEL_BYTES: usize = 256;
    if value.is_empty()
        || value.len() > MAX_LABEL_BYTES
        || value
            .bytes()
            .any(|byte| byte == 0 || byte.is_ascii_control() || byte.is_ascii_whitespace())
    {
        return Err(PrivateEventError::Bound);
    }
    Ok(())
}
