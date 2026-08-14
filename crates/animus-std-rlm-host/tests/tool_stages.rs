//! Staged RLM tool-route commit fixtures.

use animus_std_rlm_host::{
    DispatchFailure, DispatchFailureClass, ToolKitRoute, ToolRequestBinding, ToolRoute,
    ToolRouteCommitObservation, ToolRouteCommitRequest, ToolRouteCommitService,
};

const TOOL_NAME: &str = "lookup";
const TOOL_SCHEMA: &str = "schema.lookup.v1";
const TOOL_CAPABILITY: &str = "capability.lookup";
const EXECUTOR_BINDING: &str = "executor.lookup";
const ARGUMENT_NAME: &str = "key";
const OUTPUT_VISIBLE_CHARS: u32 = 64;
const OUTPUT_BYTES: u64 = 256;
const MAX_RESULT_CHARS: u32 = OUTPUT_VISIBLE_CHARS;
const BLAKE3_HEX_CHARS: usize = 64;

#[test]
fn rlm_tool_route_uses_stages_and_exact_commit() {
    // r[verify agent_harness_runtime.tool_execution_stages.commit]
    let catalog = catalog();
    let capabilities = capabilities();
    let executors = [animus_runtime::tool_executor_kit::ToolExecutorBinding::new(
        EXECUTOR_BINDING,
        animus_runtime::tool_executor_kit::ToolExecutorReadiness::Ready,
    )
    .expect("executor")];
    let authority_seal_identity = hash("authority");
    let commit_owner_identity = hash("commit-owner");
    let mut route = ToolKitRoute {
        catalog: &catalog,
        capabilities: &capabilities,
        executors: &executors,
        admission: animus_runtime::tool_executor_kit::ToolExecutorAdmission::admitted(),
        authority_seal_identity: authority_seal_identity.as_str(),
        commit_owner_identity: commit_owner_identity.as_str(),
        service: FakeExecutor { call_count: 0 },
        binder: ExactBinder,
        commit: ExactCommit {
            call_count: 0,
            complete: true,
        },
    };
    let (output, identity) = route
        .execute_tool(TOOL_NAME, "alpha", MAX_RESULT_CHARS)
        .expect("staged route");
    assert_eq!(output, "safe output");
    assert_eq!(identity.as_str().len(), BLAKE3_HEX_CHARS);
    assert_eq!(route.service.call_count, 1);
    assert_eq!(route.commit.call_count, 1);
}

#[test]
fn rlm_tool_route_rejects_partial_commit_after_one_executor_effect() {
    // r[verify agent_harness_runtime.tool_execution_stages.verification]
    let catalog = catalog();
    let capabilities = capabilities();
    let executors = [animus_runtime::tool_executor_kit::ToolExecutorBinding::new(
        EXECUTOR_BINDING,
        animus_runtime::tool_executor_kit::ToolExecutorReadiness::Ready,
    )
    .expect("executor")];
    let authority_seal_identity = hash("authority");
    let commit_owner_identity = hash("commit-owner");
    let mut route = ToolKitRoute {
        catalog: &catalog,
        capabilities: &capabilities,
        executors: &executors,
        admission: animus_runtime::tool_executor_kit::ToolExecutorAdmission::admitted(),
        authority_seal_identity: authority_seal_identity.as_str(),
        commit_owner_identity: commit_owner_identity.as_str(),
        service: FakeExecutor { call_count: 0 },
        binder: ExactBinder,
        commit: ExactCommit {
            call_count: 0,
            complete: false,
        },
    };
    let error = route
        .execute_tool(TOOL_NAME, "alpha", MAX_RESULT_CHARS)
        .expect_err("partial commit");
    assert_eq!(error.class, DispatchFailureClass::Stale);
    assert_eq!(route.service.call_count, 1);
    assert_eq!(route.commit.call_count, 1);
}

struct ExactBinder;

impl ToolRequestBinding for ExactBinder {
    fn bind_tool_request(
        &mut self,
        name: &str,
        payload: &str,
        max_result_chars: u32,
    ) -> Result<animus_adapters::ToolRequest, DispatchFailure> {
        assert_eq!(name, TOOL_NAME);
        let arguments = animus_adapters::tool::PreparedArguments::prepare(
            &animus_adapters::tool::ArgumentPreparationRequest {
                current_schema_identity: TOOL_SCHEMA,
                allowed_field_names: &[ARGUMENT_NAME],
                compatibility_rules: &[],
                fields: &[animus_adapters::tool::ArgumentFieldDraft {
                    name: Some(ARGUMENT_NAME),
                    value: Some(payload),
                }],
            },
        )
        .expect("arguments");
        Ok(
            animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
                name: Some(TOOL_NAME),
                arguments: Some(&arguments),
                correlation_id: Some("tool-call-1"),
                output_policy: animus_adapters::tool::OutputPolicy {
                    visible_char_budget: max_result_chars,
                },
                mutation_targets: &[],
                required_capability_labels: &[TOOL_CAPABILITY],
                approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved.as_str(),
            })
            .expect("request"),
        )
    }
}

struct FakeExecutor {
    call_count: u32,
}

impl animus_runtime::tool_executor_kit::ToolExecutorService for FakeExecutor {
    fn binding_label(&self) -> &str {
        EXECUTOR_BINDING
    }

    fn execute(
        &mut self,
        _plan: &animus_runtime::tool_executor_kit::ToolExecutionPlan,
        _request: &animus_adapters::ToolRequest,
    ) -> Result<
        animus_runtime::tool_executor_kit::ToolExecutorOutput,
        animus_runtime::tool_executor_kit::ToolExecutorServiceFailure,
    > {
        self.call_count = self.call_count.saturating_add(1);
        Ok(animus_runtime::tool_executor_kit::ToolExecutorOutput::new(
            "safe output",
            None,
            animus_adapters::tool::OutputRedactionClass::Safe,
        ))
    }
}

struct ExactCommit {
    call_count: u32,
    complete: bool,
}

impl ToolRouteCommitService for ExactCommit {
    fn commit(
        &mut self,
        request: ToolRouteCommitRequest<'_>,
    ) -> Result<ToolRouteCommitObservation, DispatchFailure> {
        assert_eq!(request.stage_plan.len(), BLAKE3_HEX_CHARS);
        assert_eq!(request.result.len(), BLAKE3_HEX_CHARS);
        assert_eq!(request.commit_owner.len(), BLAKE3_HEX_CHARS);
        self.call_count = self.call_count.saturating_add(1);
        Ok(ToolRouteCommitObservation {
            durable_head_identity: hash("durable-head"),
            complete: self.complete,
            synchronized: true,
        })
    }
}

fn catalog() -> animus_runtime::tool_executor_kit::ToolCatalog {
    let metadata =
        animus_adapters::tool::Definition::validate(&animus_adapters::tool::DefinitionDraft {
            name: Some(TOOL_NAME),
            label: Some("Lookup"),
            description: Some("Reads one deterministic fixture."),
            parameter_schema_identity: Some(TOOL_SCHEMA),
            prompt_snippet: Some("Use lookup for fixture reads."),
            prompt_guidelines: &["Supply one key."],
            source_identity: Some("rlm-tool-stage-tests"),
            provenance: Some("maintained-test"),
            capability_labels: &[TOOL_CAPABILITY],
            result_metadata_shape: Some("bounded-text.v1"),
            status: animus_adapters::tool::DefinitionStatus::Active,
        })
        .expect("metadata");
    let definition = animus_runtime::tool_executor_kit::ToolDefinition::from_draft(
        animus_runtime::tool_executor_kit::ToolDefinitionDraft {
            metadata,
            executor_binding: EXECUTOR_BINDING,
            argument_names: &[ARGUMENT_NAME],
            mutation_posture: animus_runtime::tool_executor_kit::ToolMutationPosture::ReadOnly,
            output_policy: animus_runtime::tool_executor_kit::ToolExecutorOutputPolicy::new(
                OUTPUT_VISIBLE_CHARS,
                OUTPUT_BYTES,
                false,
            )
            .expect("output policy"),
            approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved,
        },
    )
    .expect("definition");
    animus_runtime::tool_executor_kit::ToolCatalog::new(vec![definition]).expect("catalog")
}

fn capabilities() -> animus_runtime::RuntimeCapabilityRegistry {
    animus_runtime::RuntimeCapabilityRegistry::from_drafts(&[
        animus_runtime::RuntimeCapabilityDescriptorDraft {
            label: TOOL_CAPABILITY,
            service_kind: "tool",
            scope: None,
        },
    ])
    .expect("capabilities")
}

fn hash(label: &str) -> String {
    blake3::hash(label.as_bytes()).to_hex().to_string()
}
