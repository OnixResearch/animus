//! Worker-owned live Steel startup and durable restore composition.

use std::path::PathBuf;

/// Explicit worker request for one live Steel turn.
pub struct LiveWorkerRequest<'a> {
    /// Live Steel configuration JSON.
    pub live_config_json: &'a [u8],
    /// Exact capability-owned store root.
    pub store_root: PathBuf,
    /// One explicit prompt.
    pub prompt: &'a str,
    /// Explicit injected provider credential.
    pub credential: String,
    /// Explicit finite context values.
    pub context: animus_live_steel_agent::ExplicitContextMap,
}

/// Safe worker execution receipt. Private final text remains worker-owned.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveWorkerReceipt {
    /// Live shell metadata-only run identity.
    pub run_identity: String,
    /// Committed message count.
    pub committed_message_count: u32,
    /// Provider dispatch count.
    pub provider_dispatch_count: u32,
    /// Tool dispatch count.
    pub tool_dispatch_count: u32,
    /// Host observation count.
    pub host_observation_count: u64,
    /// Safe kernel head identity.
    pub kernel_head_identity: String,
}

/// Worker-owned output. The supervisor must project only [`LiveWorkerReceipt`].
pub struct LiveWorkerOutput {
    final_text: String,
    receipt: LiveWorkerReceipt,
}

impl LiveWorkerOutput {
    /// Returns the private final text inside the worker boundary.
    #[must_use]
    pub fn final_text(&self) -> &str {
        self.final_text.as_str()
    }

    /// Returns safe metadata for supervisor routing.
    #[must_use]
    pub const fn receipt(&self) -> &LiveWorkerReceipt {
        &self.receipt
    }
}

/// Safe live worker startup or restore failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveWorkerError {
    /// Live application startup, restore, provider, or session execution failed.
    Application,
}

impl core::fmt::Display for LiveWorkerError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "live worker failed: {self:?}")
    }
}

impl std::error::Error for LiveWorkerError {}

/// Starts or restores the live Steel service inside the worker boundary.
///
/// # Errors
///
/// Returns [`LiveWorkerError`] when strict configuration, credential, durable
/// session, kernel restore, provider, tool, child, or shutdown handling fails.
#[cfg(feature = "live-worker")]
pub fn run_live_worker(
    request: LiveWorkerRequest<'_>,
) -> Result<LiveWorkerOutput, LiveWorkerError> {
    let output = animus_live_steel_agent::run_live_application(
        animus_live_steel_agent::LiveApplicationInput {
            config_json: request.live_config_json,
            store_root: request.store_root.as_path(),
            prompt: request.prompt,
            credential: request.credential,
            context: request.context,
        },
    )
    .map_err(|_error| LiveWorkerError::Application)?;
    let receipt = output.receipt();
    let safe = LiveWorkerReceipt {
        run_identity: blake3::hash(receipt.render().as_bytes())
            .to_hex()
            .to_string(),
        committed_message_count: receipt.committed_message_count(),
        provider_dispatch_count: receipt.provider_dispatch_count(),
        tool_dispatch_count: receipt.tool_dispatch_count(),
        host_observation_count: receipt.host_observation_count(),
        kernel_head_identity: String::from(receipt.kernel_head_identity()),
    };
    Ok(LiveWorkerOutput {
        final_text: String::from(output.final_text()),
        receipt: safe,
    })
}

/// Explicit feature-disabled worker failure.
///
/// # Errors
///
/// Always returns [`LiveWorkerError::Application`] when live worker support is absent.
#[cfg(not(feature = "live-worker"))]
pub fn run_live_worker(
    _request: LiveWorkerRequest<'_>,
) -> Result<LiveWorkerOutput, LiveWorkerError> {
    Err(LiveWorkerError::Application)
}
