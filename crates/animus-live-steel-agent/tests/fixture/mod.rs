mod bridge;
mod commands;
mod completion;
mod persistence;
mod retries;
// Deterministic production-entrypoint and resumable-host tests.

pub(super) const CONFIG: &[u8] =
    include_bytes!("../../../../config/generated/live-steel-agent-shell.json");
pub(super) const TOOL_STREAM: &[u8] =
    include_bytes!("../../../animus-std-openai/tests/fixtures/openai-responses-tool-call.sse");
pub(super) const FINAL_STREAM: &[u8] =
    include_bytes!("../../../animus-std-openai/tests/fixtures/openai-responses-final.sse");
pub(super) const EXPECTED_MESSAGES: u32 = 5;
pub(super) const EXPECTED_PROVIDER_DISPATCHES: u32 = 2;
pub(super) const EXPECTED_TOOL_DISPATCHES: u32 = 1;
pub(super) const EXPECTED_ZERO_HOST_EFFECTS: u64 = 0;
pub(super) const CORRELATION_A: &str =
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
pub(super) const CORRELATION_B: &str =
    "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
pub(super) const INVALID_CORRELATION: &str = "not-a-blake3-identity";

#[derive(Default)]
pub(super) struct ContextExecutor {
    calls: u32,
}

impl animus_runtime::rlm_kernel::RlmKernelHostExecutor for ContextExecutor {
    type Error = &'static str;

    fn execute(
        &mut self,
        request: &animus_adapter_rlm_kernel::HostRequest,
    ) -> Result<String, Self::Error> {
        match request {
            animus_adapter_rlm_kernel::HostRequest::ContextRead { key } if key == "fixture-key" => {
                self.calls = self.calls.saturating_add(1);
                Ok(String::from("fixture-value"))
            }
            _other => Err("unexpected fixture request"),
        }
    }
}

impl animus_live_steel_agent::LiveSessionExecutorControl for ContextExecutor {
    type Error = &'static str;

    fn answer_approval(
        &mut self,
        correlation_id: &str,
        _decision: animus_live_steel_agent::LiveApprovalDecision,
    ) -> Result<(), Self::Error> {
        if correlation_id == CORRELATION_A {
            Ok(())
        } else {
            Err("unknown approval")
        }
    }

    fn child_status(
        &mut self,
        child_id: &str,
    ) -> Result<animus_live_steel_agent::LiveChildStatus, Self::Error> {
        if child_id == "child-active" {
            Ok(animus_live_steel_agent::LiveChildStatus::Active)
        } else {
            Err("unknown child")
        }
    }

    fn cancel_child(
        &mut self,
        child_id: &str,
    ) -> Result<animus_live_steel_agent::LiveChildStatus, Self::Error> {
        if child_id == "child-active" {
            Ok(animus_live_steel_agent::LiveChildStatus::Cancelled)
        } else {
            Err("unknown child")
        }
    }
}
