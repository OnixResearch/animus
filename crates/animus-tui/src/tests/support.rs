const RUN_ID: &str = crate::bridge::test_support::RUN_ID;
const CORRELATION_ID: &str = crate::bridge::test_support::CORRELATION_ID;
const APPROVAL_PROMPT: &str = crate::bridge::test_support::APPROVAL_PROMPT;
const TOOL_NAME: &str = crate::bridge::test_support::TOOL_NAME;
const TOOL_INPUT: &str = crate::bridge::test_support::TOOL_INPUT;
const TOOL_OUTPUT: &str = crate::bridge::test_support::TOOL_OUTPUT;
const LONG_OUTPUT: &str = crate::bridge::test_support::LONG_OUTPUT;
const BAD_NUL: &str = crate::bridge::test_support::BAD_NUL;
const MISMATCHED_CORRELATION_ID: &str = crate::bridge::test_support::MISMATCHED_CORRELATION_ID;
const COMPLETED_UNITS: u32 = crate::bridge::test_support::COMPLETED_UNITS;
const TOTAL_UNITS: u32 = crate::bridge::test_support::TOTAL_UNITS;
const TOO_MANY_COMPLETED_UNITS: u32 = crate::bridge::test_support::TOO_MANY_COMPLETED_UNITS;
const OUTPUT_BUDGET: u32 = crate::bridge::test_support::OUTPUT_BUDGET;
const SINK_FAILURE: &str = "sink failed";
const COMMAND_FAILURE: &str = "commands closed";
const SNAPSHOT_WAITING: &str = "session: waiting_for_approval";
const SNAPSHOT_RUNNING: &str = "session: running";
const SNAPSHOT_TOOL: &str = "tool: search";
const SNAPSHOT_PROGRESS: &str = "progress: 1/3";
const SNAPSHOT_OUTPUT: &str = "output: alpha";
const SNAPSHOT_APPROVAL: &str = "approval: allow guarded action";
const SNAPSHOT_CHOICES: &str = "choices: approve,deny";
const SNAPSHOT_FINAL: &str = "final: tool output";
const SNAPSHOT_FAILURE_DENIED: &str = "failure: denied:denied by policy";
const DENIED_REASON: &str = "denied by policy";
const EXPECTED_CHOICE_COUNT: usize = 2;

fn approval_request(choices: &[crate::ToolCommandChoice]) -> crate::ApprovalRequest {
    crate::ApprovalRequest::new(
        crate::ToolRunId::new(RUN_ID).expect("run ID should validate"),
        crate::ToolCorrelationId::new(CORRELATION_ID).expect("correlation should validate"),
        APPROVAL_PROMPT,
        choices,
    )
    .expect("approval request should validate")
}

fn tool_request() -> animus_adapters::ToolRequest {
    animus_adapters::ToolRequest::new(TOOL_NAME, TOOL_INPUT).expect("tool request should validate")
}

fn tool_response() -> animus_adapters::ToolResponse {
    animus_adapters::ToolResponse::new(TOOL_OUTPUT).expect("tool response should validate")
}

fn bounded_output() -> animus_adapters::tool::BoundedOutput {
    animus_adapters::tool::BoundedOutput::new(
        &animus_adapters::tool::OutputDraft {
            body: LONG_OUTPUT,
            full_output_reference: None,
            hash_requirement: animus_adapters::tool::HashRequirement::Optional,
        },
        animus_adapters::tool::OutputPolicy {
            visible_char_budget: OUTPUT_BUDGET,
        },
    )
    .expect("bounded output should validate")
}

const fn expected_choice_count() -> usize {
    EXPECTED_CHOICE_COUNT
}

const fn expected_call_count() -> usize {
    1
}

const fn no_call_count() -> usize {
    0
}

#[derive(Default)]
struct CountingTool {
    calls: usize,
}

impl animus_adapters::ComponentAdapter<animus_adapters::ToolRequest, animus_adapters::ToolResponse>
    for CountingTool
{
    type Error = animus_adapters::AdapterContractError;

    fn call(
        &mut self,
        _request: &animus_adapters::ToolRequest,
    ) -> Result<animus_adapters::ToolResponse, Self::Error> {
        self.calls = self.calls.saturating_add(expected_call_count());
        Ok(tool_response())
    }
}

#[derive(Default)]
struct RecordingSink {
    events: Vec<crate::ToolHostEvent>,
}

impl crate::ToolHostEventSink for RecordingSink {
    type Error = String;

    fn emit(&mut self, event: crate::ToolHostEvent) -> Result<(), Self::Error> {
        self.events.push(event);
        Ok(())
    }
}

struct FailingSink;

impl crate::ToolHostEventSink for FailingSink {
    type Error = String;

    fn emit(&mut self, _event: crate::ToolHostEvent) -> Result<(), Self::Error> {
        Err(String::from(SINK_FAILURE))
    }
}

struct StaticCommandSource {
    command: crate::ToolHostCommand,
}

impl StaticCommandSource {
    const fn new(command: crate::ToolHostCommand) -> Self {
        Self { command }
    }
}

impl crate::ToolHostCommandSource for StaticCommandSource {
    type Error = String;

    fn decide(
        &mut self,
        _request: &crate::ApprovalRequest,
    ) -> Result<crate::ToolHostCommand, Self::Error> {
        Ok(self.command.clone())
    }
}

struct FailingCommandSource;

impl crate::ToolHostCommandSource for FailingCommandSource {
    type Error = String;

    fn decide(
        &mut self,
        _request: &crate::ApprovalRequest,
    ) -> Result<crate::ToolHostCommand, Self::Error> {
        Err(String::from(COMMAND_FAILURE))
    }
}
