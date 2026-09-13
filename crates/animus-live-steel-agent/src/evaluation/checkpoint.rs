pub(super) fn map_bridge_error<HostError>(
    error: &animus_runtime::rlm_kernel::RlmBridgeExecutionError<HostError>,
) -> super::SteelToolError {
    match error.detail() {
        animus_runtime::rlm_kernel::RlmBridgeErrorDetail::Execution(_) => {
            super::SteelToolError::Host
        }
        animus_runtime::rlm_kernel::RlmBridgeErrorDetail::Contract(
            animus_adapter_rlm_kernel::Error::BridgeCancelled,
        ) => super::SteelToolError::Cancelled,
        animus_runtime::rlm_kernel::RlmBridgeErrorDetail::Contract(_)
        | animus_runtime::rlm_kernel::RlmBridgeErrorDetail::Steel(_) => {
            super::SteelToolError::Evaluation
        }
    }
}

pub(super) struct ExplicitBridgeControl {
    pub(super) cancelled: bool,
    pub(super) clock: cap_std::time::MonotonicClock,
    pub(super) started: cap_std::time::Instant,
}

impl animus_runtime::rlm_kernel::RlmBridgeControl for ExplicitBridgeControl {
    fn observe(
        &mut self,
        _checkpoint: animus_runtime::rlm_kernel::RlmBridgeCheckpoint,
    ) -> animus_runtime::rlm_kernel::RlmBridgeControlObservation {
        let elapsed_ms = u64::try_from(self.clock.elapsed(self.started).as_millis());
        observation(self.cancelled, elapsed_ms)
    }
}

const fn observation(
    cancelled: bool,
    elapsed_ms: Result<u64, std::num::TryFromIntError>,
) -> animus_runtime::rlm_kernel::RlmBridgeControlObservation {
    match elapsed_ms {
        Ok(elapsed_ms) => animus_runtime::rlm_kernel::RlmBridgeControlObservation {
            cancelled,
            elapsed_ms,
        },
        Err(_error) => animus_runtime::rlm_kernel::RlmBridgeControlObservation {
            // An unrepresentable duration cannot admit another host effect.
            cancelled: true,
            elapsed_ms: 0,
        },
    }
}

pub(super) fn validate_request(
    request: &animus_adapters::ToolRequest,
) -> Result<(), super::SteelToolError> {
    let is_exact = request.name() == animus_std_openai::STEEL_REPL_TOOL_NAME
        && request.schema_identity() == animus_std_openai::STEEL_REPL_TOOL_SCHEMA
        && request.approval_posture() != animus_adapters::tool::ApprovalPosture::Denied
        && request.arguments().len() == 1
        && request.arguments()[0].name() == "code";
    if !is_exact {
        return Err(super::SteelToolError::Request);
    }
    Ok(())
}

pub(super) fn hash_parts(domain: &str, parts: &[&str]) -> Result<String, super::SteelToolError> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    for part in parts {
        let length_bytes =
            u64::try_from(part.len()).map_err(|_error| super::SteelToolError::Request)?;
        hasher.update(&length_bytes.to_le_bytes());
        hasher.update(part.as_bytes());
    }
    Ok(hasher.finalize().to_hex().to_string())
}

#[cfg(test)]
mod tests {
    const ELAPSED_MS: u64 = 17;

    #[test]
    fn representable_time_retains_the_explicit_cancellation_state() {
        for cancelled in [false, true] {
            let observed = super::observation(cancelled, Ok(ELAPSED_MS));
            assert_eq!(observed.cancelled, cancelled);
            assert_eq!(observed.elapsed_ms, ELAPSED_MS);
        }
    }

    #[test]
    fn the_injected_clock_counts_prior_elapsed_time() {
        use animus_runtime::rlm_kernel::RlmBridgeControl;

        let clock = cap_std::time::MonotonicClock::new(cap_std::ambient_authority());
        let elapsed = cap_std::time::Duration::from_millis(ELAPSED_MS);
        let started = clock.now().checked_sub(elapsed).expect("past instant fits");
        let mut control = super::ExplicitBridgeControl {
            cancelled: false,
            clock,
            started,
        };
        let observed = control.observe(animus_runtime::rlm_kernel::RlmBridgeCheckpoint::Admission);
        assert!(!observed.cancelled);
        assert!(observed.elapsed_ms >= ELAPSED_MS);
    }

    #[test]
    fn an_injected_clock_does_not_clear_explicit_cancellation() {
        use animus_runtime::rlm_kernel::RlmBridgeControl;

        let clock = cap_std::time::MonotonicClock::new(cap_std::ambient_authority());
        let started = clock.now();
        let mut control = super::ExplicitBridgeControl {
            cancelled: true,
            clock,
            started,
        };
        let observed = control.observe(animus_runtime::rlm_kernel::RlmBridgeCheckpoint::Admission);
        assert!(observed.cancelled);
    }

    #[test]
    fn unrepresentable_time_cannot_admit_another_effect() {
        let elapsed_ms = u64::try_from(u128::MAX);
        let observed = super::observation(false, elapsed_ms);
        assert!(observed.cancelled);
        assert_eq!(observed.elapsed_ms, 0);
    }
}
