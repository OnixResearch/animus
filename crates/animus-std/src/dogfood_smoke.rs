use animus_runtime::dogfood_shell::{
    DogfoodFakeServices, DogfoodReceipt, DogfoodRunRequest, DogfoodServiceState,
    run_fake_dogfood_shell,
};

const ARG_PROMPT: &str = "--prompt";
const ARG_POLICY: &str = "--policy";
const ARG_MODEL: &str = "--model";
const ARG_OUTPUT: &str = "--output";
const ARG_TOOL: &str = "--tool";
const ARG_MISSING_MODEL_SERVICE: &str = "--missing-model-service";
const ARG_DENY_CAPABILITY: &str = "--deny-capability";
const ARG_MALFORMED_MODEL_OUTPUT: &str = "--malformed-model-output";
const DEFAULT_POLICY_PROFILE: &str = "local-packaged-fake-policy";
const DEFAULT_MODEL_CAPABILITY: &str = "fake-model-text";
const DEFAULT_OUTPUT_CONTRACT: &str = "text-summary";
const DEFAULT_MODEL_SUMMARY: &str = "packaged-fake-model-summary";
const DEFAULT_TOOL_SUMMARY: &str = "packaged-fake-tool-summary";
const PACKAGED_SMOKE_SCHEMA: &str = "animus.packaged-headless-dogfood-smoke.v1";
const PACKAGED_HEADLESS_RUNG: &str = "packaged-headless-smoke";
const CORE_RUNG_FAKE_BACKED: &str = "fake-backed-integration";
const STATUS_COMPLETED: &str = "completed";
const STATUS_FAILED: &str = "failed";
const HASH_DOMAIN: &str = "animus.packaged-headless-dogfood-smoke.prompt.v1";
const HASH_SEPARATOR: &str = "|";
const RECORD_HEADER: &str = "ANIMUS_DOGFOOD_SMOKE";
const RECORD_EVENT: &str = "EVENT";
const RECORD_NON_CLAIM: &str = "NON_CLAIM";
const ERROR_MISSING_PROMPT: &str = "missing-prompt";
const ERROR_UNKNOWN_ARGUMENT: &str = "unknown-argument";
const ERROR_MISSING_ARGUMENT_VALUE: &str = "missing-argument-value";
const ERROR_CORE_ADMISSION: &str = "core-admission";
const ERROR_CORE_RUNG: &str = "core-rung";

#[derive(Clone, Debug, Eq, PartialEq)]
struct SmokeConfig {
    prompt: String,
    policy_profile: String,
    model_capability: String,
    output_contract: String,
    tool_metadata: Option<String>,
    model_service: DogfoodServiceState,
    allowed_capabilities: Vec<String>,
    model_response_summary: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SmokeReport {
    prompt_hash: String,
    receipt: DogfoodReceipt,
    events: Vec<animus_runtime::dogfood_shell::DogfoodEvent>,
}

/// Run the packaged non-default headless dogfood smoke from already-loaded CLI arguments.
///
/// The returned text is a safe line-oriented receipt projection. It contains hashes, status
/// classes, event counts, adapter labels, and omitted-surface non-claims; it intentionally omits
/// the raw prompt and raw model/tool outputs.
///
/// # Errors
///
/// Returns an error string when arguments are malformed, required prompt input is absent, the
/// fake-backed core rejects admission, or the core receipt attempts to claim a rung other than the
/// expected fake-backed integration evidence.
pub fn run_packaged_headless_dogfood_smoke(args: &[String]) -> Result<String, String> {
    let config = parse_smoke_args(args)?;
    let report = run_smoke(&config)?;
    Ok(render_report(&report))
}

fn parse_smoke_args(args: &[String]) -> Result<SmokeConfig, String> {
    let mut cursor = ArgCursor::new(args);
    let mut prompt = None;
    let mut policy_profile = DEFAULT_POLICY_PROFILE.to_owned();
    let mut model_capability = DEFAULT_MODEL_CAPABILITY.to_owned();
    let mut output_contract = DEFAULT_OUTPUT_CONTRACT.to_owned();
    let mut tool_metadata = None;
    let mut model_service = DogfoodServiceState::Available;
    let mut deny_capability = false;
    let mut model_response_summary = DEFAULT_MODEL_SUMMARY.to_owned();

    while let Some(argument) = cursor.next() {
        match argument.as_str() {
            ARG_PROMPT => prompt = Some(cursor.next_value(ARG_PROMPT)?),
            ARG_POLICY => policy_profile = cursor.next_value(ARG_POLICY)?,
            ARG_MODEL => model_capability = cursor.next_value(ARG_MODEL)?,
            ARG_OUTPUT => output_contract = cursor.next_value(ARG_OUTPUT)?,
            ARG_TOOL => tool_metadata = Some(cursor.next_value(ARG_TOOL)?),
            ARG_MISSING_MODEL_SERVICE => model_service = DogfoodServiceState::Missing,
            ARG_DENY_CAPABILITY => deny_capability = true,
            ARG_MALFORMED_MODEL_OUTPUT => model_response_summary = "MALFORMED".to_owned(),
            other => return Err(format_error(ERROR_UNKNOWN_ARGUMENT, other)),
        }
    }

    let prompt = prompt.ok_or_else(|| format_error(ERROR_MISSING_PROMPT, ARG_PROMPT))?;
    let allowed_capabilities = if deny_capability {
        Vec::new()
    } else {
        vec![model_capability.clone()]
    };

    Ok(SmokeConfig {
        prompt,
        policy_profile,
        model_capability,
        output_contract,
        tool_metadata,
        model_service,
        allowed_capabilities,
        model_response_summary,
    })
}

fn run_smoke(config: &SmokeConfig) -> Result<SmokeReport, String> {
    let prompt_hash = prompt_hash(&config.prompt);
    let request = DogfoodRunRequest {
        prompt_hash: prompt_hash.clone(),
        policy_profile: config.policy_profile.clone(),
        model_capability: config.model_capability.clone(),
        output_contract: config.output_contract.clone(),
        tool_metadata: config.tool_metadata.clone(),
        noninteractive: true,
    };
    let services = DogfoodFakeServices {
        model_service: config.model_service,
        tool_service: DogfoodServiceState::Available,
        allowed_capabilities: config.allowed_capabilities.clone(),
        model_response_summary: config.model_response_summary.clone(),
        tool_response_summary: config
            .tool_metadata
            .as_ref()
            .map(|_| DEFAULT_TOOL_SUMMARY.to_owned()),
    };

    let (events, receipt) = run_fake_dogfood_shell(&request, &services).map_err(render_issues)?;
    if receipt.evidence_rung != CORE_RUNG_FAKE_BACKED {
        return Err(format_error(
            ERROR_CORE_RUNG,
            receipt.evidence_rung.as_str(),
        ));
    }

    Ok(SmokeReport {
        prompt_hash,
        receipt,
        events,
    })
}

fn render_report(report: &SmokeReport) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "{RECORD_HEADER} schema={PACKAGED_SMOKE_SCHEMA} status={STATUS_COMPLETED} packaged_rung={PACKAGED_HEADLESS_RUNG} core_rung={} event_count={} prompt_hash={} receipt_hash={}",
        report.receipt.evidence_rung,
        report.receipt.event_count,
        report.prompt_hash,
        report.receipt.receipt_hash
    ));
    for event in &report.events {
        lines.push(format!(
            "{RECORD_EVENT} sequence={} kind={} label_hash={}",
            event.sequence, event.kind, event.label_hash
        ));
    }
    for surface in &report.receipt.omitted_surfaces {
        lines.push(format!("{RECORD_NON_CLAIM} surface={surface}"));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn render_issues(report: animus_runtime::dogfood_shell::DogfoodValidationReport) -> String {
    let issue_codes = report
        .issues
        .iter()
        .map(|issue| issue.code())
        .collect::<Vec<_>>()
        .join(",");
    format_error(ERROR_CORE_ADMISSION, issue_codes.as_str())
}

fn prompt_hash(prompt: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(HASH_DOMAIN.as_bytes());
    hasher.update(HASH_SEPARATOR.as_bytes());
    hasher.update(prompt.as_bytes());
    hasher.finalize().to_hex().to_string()
}

fn format_error(kind: &str, detail: &str) -> String {
    format!("{STATUS_FAILED}:{kind}:{detail}")
}

struct ArgCursor<'a> {
    args: &'a [String],
    index: usize,
}

impl<'a> ArgCursor<'a> {
    fn new(args: &'a [String]) -> Self {
        Self { args, index: 0 }
    }

    fn next(&mut self) -> Option<&'a String> {
        let argument = self.args.get(self.index)?;
        self.index += 1;
        Some(argument)
    }

    fn next_value(&mut self, argument: &str) -> Result<String, String> {
        let value = self
            .next()
            .ok_or_else(|| format_error(ERROR_MISSING_ARGUMENT_VALUE, argument))?;
        Ok(value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_runtime::dogfood_shell::DogfoodIssue;

    const PROMPT: &str = "bounded prompt fixture";
    const TOOL_LABEL: &str = "fake-readonly-tool";
    const UNSAFE_POLICY: &str = "RAW_PROMPT:secret";

    #[test]
    fn packaged_smoke_renders_safe_report_without_raw_prompt() {
        // r[verify agent_harness_runtime.packaged_headless_dogfood_smoke.cli]
        // r[verify agent_harness_runtime.packaged_headless_dogfood_smoke.receipt]
        let args = args(&[ARG_PROMPT, PROMPT, ARG_TOOL, TOOL_LABEL]);

        let output = run_packaged_headless_dogfood_smoke(&args).expect("packaged smoke output");

        assert!(output.contains(RECORD_HEADER));
        assert!(output.contains("packaged_rung=packaged-headless-smoke"));
        assert!(output.contains("core_rung=fake-backed-integration"));
        assert!(output.contains("event_count=6"));
        assert!(output.contains("kind=tool-admission"));
        assert!(output.contains("surface=live-provider"));
        assert!(!output.contains(PROMPT));
        assert!(!output.contains("RAW_PROMPT"));
    }

    #[test]
    fn packaged_smoke_fails_closed_for_missing_prompt_and_services() {
        // r[verify agent_harness_runtime.packaged_headless_dogfood_smoke.fail_closed]
        let missing_prompt = run_packaged_headless_dogfood_smoke(&[]).expect_err("missing prompt");
        assert!(missing_prompt.contains(ERROR_MISSING_PROMPT));

        let missing_model = run_packaged_headless_dogfood_smoke(&args(&[
            ARG_PROMPT,
            PROMPT,
            ARG_MISSING_MODEL_SERVICE,
        ]))
        .expect_err("missing model service");
        assert!(missing_model.contains(DogfoodIssue::MissingModelService.code()));
    }

    #[test]
    fn packaged_smoke_rejects_denied_capability_malformed_output_and_unsafe_labels() {
        // r[verify agent_harness_runtime.packaged_headless_dogfood_smoke.negative_fixtures]
        let denied =
            run_packaged_headless_dogfood_smoke(&args(&[ARG_PROMPT, PROMPT, ARG_DENY_CAPABILITY]))
                .expect_err("denied capability");
        assert!(denied.contains(DogfoodIssue::DeniedCapability.code()));

        let malformed = run_packaged_headless_dogfood_smoke(&args(&[
            ARG_PROMPT,
            PROMPT,
            ARG_MALFORMED_MODEL_OUTPUT,
        ]))
        .expect_err("malformed output");
        assert!(malformed.contains(DogfoodIssue::MalformedOutput.code()));

        let unsafe_label = run_packaged_headless_dogfood_smoke(&args(&[
            ARG_PROMPT,
            PROMPT,
            ARG_POLICY,
            UNSAFE_POLICY,
        ]))
        .expect_err("unsafe policy label");
        assert!(unsafe_label.contains(DogfoodIssue::UnsafeEvidence.code()));
    }

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }
}
