//! External implementations need the public stream error to preserve denial.

use animus_live_steel_agent::ProviderStreamService;

type TestResult<Value = ()> = Result<Value, Box<dyn std::error::Error>>;
type Calls = std::sync::Arc<std::sync::atomic::AtomicU32>;

struct ExternalProvider {
    inner: super::FixtureStreamProvider,
    calls: Calls,
    is_available: bool,
}

impl ProviderStreamService for ExternalProvider {
    fn stream(
        &mut self,
        plan: &animus_std::ProviderDispatchPlan,
        dispatch: animus_std_openai::OpenAiToolDispatch,
    ) -> Result<animus_std_openai::OpenAiStreamOutcome, animus_live_steel_agent::ProviderStreamError>
    {
        if !self.is_available {
            return Err(animus_live_steel_agent::ProviderStreamError::Unavailable);
        }
        self.is_available = false;
        self.calls.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        self.inner.stream(plan, dispatch)
    }
}

fn run(
    streams: &[&[u8]],
    is_available: bool,
    calls: Calls,
) -> TestResult<animus_live_steel_agent::LiveSteelAgentRunOutput> {
    let config = super::parse_live_steel_agent_config(super::CONFIG)?;
    debug_assert!(config.max_provider_response_bytes() > 0);
    debug_assert!(config.max_output_tokens() > 0);
    let inner = super::FixtureStreamProvider::new(
        streams.iter().map(|stream| stream.to_vec()),
        usize::try_from(config.max_provider_response_bytes())?,
    );
    let kernel = animus_steel::SteelRlmKernel::new(config.rlm_policy().clone())
        .map_err(|_| "fixture kernel")?;
    let composition =
        super::LiveSteelAgentComposition::new(super::LiveSteelAgentCompositionInput {
            config,
            provider: ExternalProvider {
                inner,
                calls,
                is_available,
            },
            session_persistence: super::MemoryPersistence::default(),
            snapshot_persistence: super::MemorySnapshotPersistence::default(),
            kernel,
            host_executor: super::ContextExecutor::default(),
            kernel_persistence: (),
        })?;
    Ok(composition.run_prompt("Return the fixture answer.")?)
}

#[test]
fn an_external_provider_can_forward_one_stream() -> TestResult {
    // r[verify agent_harness_runtime.live_steel_agent_shell.verification]
    let calls = Calls::new(std::sync::atomic::AtomicU32::new(0));
    let output = run(&[super::FINAL_STREAM], true, Calls::clone(&calls))?;
    assert_eq!(output.final_text(), "3");
    assert_eq!(calls.load(std::sync::atomic::Ordering::Acquire), 1);
    Ok(())
}

#[test]
fn external_denial_prevents_the_inner_provider_call() {
    let calls = Calls::new(std::sync::atomic::AtomicU32::new(0));
    assert!(run(&[super::FINAL_STREAM], false, Calls::clone(&calls)).is_err());
    assert_eq!(calls.load(std::sync::atomic::Ordering::Acquire), 0);
}

#[test]
fn external_exhaustion_prevents_a_second_stream() {
    let calls = Calls::new(std::sync::atomic::AtomicU32::new(0));
    assert!(
        run(
            &[super::TOOL_STREAM, super::FINAL_STREAM],
            true,
            Calls::clone(&calls)
        )
        .is_err()
    );
    assert_eq!(calls.load(std::sync::atomic::Ordering::Acquire), 1);
}
