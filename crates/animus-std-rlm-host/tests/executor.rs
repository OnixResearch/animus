//! Production RLM host executor conformance tests.

use std::cell::Cell;
use std::path::Path;
use std::sync::atomic::AtomicBool;

const PROCESS_FIXTURE_ENV: &str = "ANIMUS_RLM_HOST_PROCESS_FIXTURE";

use animus_std_rlm_host::{
    BindingIdentity, BindingSet, CapabilityPacks, CurrentAuthorityObservation,
    CurrentAuthorityObserver, DispatchFailure, DispatchFailureClass, EditRoute, ExecutorLimits,
    ExpectedAuthority, FileReadPack, KernelClearRoute, ProcessPack, ProcessProfileSet,
    ResolverContextRoute, RlmHostExecutor, RlmHostExecutorConfig, RouteBinding, RouteClass,
    ToolRoute,
};

const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const HASH_C: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
const HASH_D: &str = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
const HASH_E: &str = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
const HASH_F: &str = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
const GENERATION: u64 = 1;
const FILE_NAME: &str = "input.txt";
const FILE_BODY: &str = "bounded file\n";
const CONTEXT_KEY: &str = "@context.txt";
const CONTEXT_BODY: &str = "bounded context\n";
const TOOL_NAME: &str = "echo";
const TOOL_PAYLOAD: &str = "payload";
const TOOL_RESULT: &str = "bounded tool";
const CHILD_RESULT: &str = "bounded child";
const RESULT_CHAR_LIMIT: u32 = 4_096;
const FILE_BYTE_LIMIT: u64 = 4_096;
const CONTEXT_BYTE_LIMIT: u32 = 4_096;
const CONTEXT_LINE_LIMIT: u32 = 64;
const CHILD_STATUS_LIMIT: u32 = 8;
const TIMEOUT_SECONDS: u32 = 5;
const LOG_BYTES: u64 = 4_096;

#[derive(Clone)]
struct RecordingAuthority {
    expected: ExpectedAuthority,
    stale_generation: bool,
    cancelled: bool,
    timed_out: bool,
}

impl CurrentAuthorityObserver for RecordingAuthority {
    fn observe(
        &mut self,
        _route: RouteClass,
        binding: &RouteBinding,
        request_scope: &BindingIdentity,
    ) -> Result<CurrentAuthorityObservation, DispatchFailure> {
        let unit = &self.expected.unit;
        Ok(CurrentAuthorityObservation {
            session_lineage_identity: unit.session_lineage_identity().to_owned(),
            authority_profile_identity: unit.authority_profile_identity().to_owned(),
            durable_binding_identity: unit.durable_binding_identity().to_owned(),
            enforcement_observation_identity: unit.enforcement_observation_identity().to_owned(),
            operation_set_identity: unit.operation_set_identity().to_owned(),
            adapter_cohort_identity: unit.adapter_cohort_identity().to_owned(),
            resource_profile_identity: unit.resource_profile_identity().to_owned(),
            resource_budget_identity: unit.resource_budget_identity().to_owned(),
            runtime_services_identity: unit.runtime_services_identity().to_owned(),
            operation_identities: unit.operation_identities().to_vec(),
            resource_identities: unit.resource_identities().to_vec(),
            binding_identity: binding.identity().clone(),
            binding_scope_identity: binding.scope_identity().clone(),
            request_scope_identity: request_scope.clone(),
            generation: if self.stale_generation {
                binding.generation().saturating_add(1)
            } else {
                binding.generation()
            },
            admitted: true,
            cancelled: self.cancelled,
            timed_out: self.timed_out,
        })
    }
}

struct FakeEdit(u32);
impl EditRoute for FakeEdit {
    fn execute_edit(
        &mut self,
        target: &str,
        replacement: &str,
    ) -> Result<BindingIdentity, DispatchFailure> {
        assert_eq!(target, FILE_NAME);
        assert_eq!(replacement, "replacement");
        self.0 = self.0.saturating_add(1);
        Ok(BindingIdentity::new(HASH_D).expect("fixture hash"))
    }
}

struct FakeTool(u32);
impl ToolRoute for FakeTool {
    fn execute_tool(
        &mut self,
        name: &str,
        payload: &str,
        _max_result_chars: u32,
    ) -> Result<(String, BindingIdentity), DispatchFailure> {
        assert_eq!(name, TOOL_NAME);
        assert_eq!(payload, TOOL_PAYLOAD);
        self.0 = self.0.saturating_add(1);
        Ok((
            String::from(TOOL_RESULT),
            BindingIdentity::new(HASH_E).expect("fixture hash"),
        ))
    }
}

struct FailingTool;
impl ToolRoute for FailingTool {
    fn execute_tool(
        &mut self,
        _name: &str,
        _payload: &str,
        _max_result_chars: u32,
    ) -> Result<(String, BindingIdentity), DispatchFailure> {
        Err(DispatchFailure::new(
            RouteClass::Tool,
            DispatchFailureClass::OwnerFailure,
            "tool_executor",
        ))
    }
}

struct OversizedTool;
impl ToolRoute for OversizedTool {
    fn execute_tool(
        &mut self,
        _name: &str,
        _payload: &str,
        _max_result_chars: u32,
    ) -> Result<(String, BindingIdentity), DispatchFailure> {
        Ok((
            "private-tool-output".repeat(RESULT_CHAR_LIMIT as usize),
            BindingIdentity::new(HASH_E).expect("fixture hash"),
        ))
    }
}

struct FakeChild(u32);
impl animus_std_rlm_host::ChildRoute for FakeChild {
    fn execute_child(
        &mut self,
        request: &animus_adapter_rlm_kernel::ChildSpawnRequest,
        _max_status_checks: u32,
    ) -> Result<(String, BindingIdentity), DispatchFailure> {
        assert_eq!(request.profile(), "focused-child");
        self.0 = self.0.saturating_add(1);
        Ok((
            String::from(CHILD_RESULT),
            BindingIdentity::new(HASH_F).expect("fixture hash"),
        ))
    }
}

struct FakeKernel(u32);
impl KernelClearRoute for FakeKernel {
    fn clear_kernel(&mut self) -> Result<BindingIdentity, DispatchFailure> {
        self.0 = self.0.saturating_add(1);
        Ok(BindingIdentity::new(HASH_C).expect("fixture hash"))
    }
}

struct FileResolver {
    body: String,
    calls: Cell<u32>,
}
impl animus_adapters::FileContextResolver for FileResolver {
    fn resolve_file(
        &self,
        reference: &animus_adapters::ContextReference,
    ) -> Result<animus_adapters::ResolvedContextCandidate, animus_adapters::ContextReferenceError>
    {
        self.calls.set(self.calls.get().saturating_add(1));
        Ok(animus_adapters::ResolvedContextCandidate::new(
            reference.clone(),
            animus_adapters::ContextResolverKind::File,
            self.body.clone(),
        ))
    }
}

#[test]
fn routes_every_operation_to_its_exact_owner() {
    // r[verify agent_harness_runtime.rlm_host_executor.verification]
    let temp = tempfile::tempdir().expect("temporary root");
    std::fs::write(temp.path().join(FILE_NAME), FILE_BODY).expect("fixture file");
    let file_dir = cap_std::fs::Dir::open_ambient_dir(temp.path(), cap_std::ambient_authority())
        .expect("capability root");
    let context_resolver = FileResolver {
        body: String::from(CONTEXT_BODY),
        calls: Cell::new(0),
    };
    let mut context = ResolverContextRoute {
        resolvers: animus_adapters::ContextResolverSet::empty().with_file(&context_resolver),
        policy: animus_adapters::ContextReferencePolicy::allow_local(),
    };
    let mut edit = FakeEdit(0);
    let mut tool = FakeTool(0);
    let mut child = FakeChild(0);
    let mut kernel = FakeKernel(0);
    let binding_values = all_bindings();
    let bindings = BindingSet::new(binding_values.clone()).expect("bindings");
    let expected = expected_authority();
    let file_binding = select_binding(&binding_values, RouteClass::FileRead);
    let process_binding = select_binding(&binding_values, RouteClass::Shell);
    let file_read = FileReadPack::new(file_dir, file_binding).expect("file pack");
    let process = ProcessPack::new(
        process_binding,
        process_profiles(temp.path()),
        AtomicBool::new(false),
    )
    .expect("process pack");
    let mut executor = RlmHostExecutor::new(RlmHostExecutorConfig {
        bindings,
        expected_authority: expected.clone(),
        authority: authority(expected),
        limits: limits(),
        packs: CapabilityPacks {
            file_read: Some(file_read),
            file_edit: Some(&mut edit),
            process: Some(process),
            tool: Some(&mut tool),
            context: Some(&mut context),
            child: Some(&mut child),
            kernel: Some(&mut kernel),
        },
    })
    .expect("executor");

    let requests = [
        animus_adapter_rlm_kernel::HostRequest::file_read(FILE_NAME).expect("file request"),
        animus_adapter_rlm_kernel::HostRequest::file_edit(
            FILE_NAME,
            "replacement",
            RESULT_CHAR_LIMIT,
        )
        .expect("edit request"),
        animus_adapter_rlm_kernel::HostRequest::shell("print", RESULT_CHAR_LIMIT)
            .expect("process request"),
        animus_adapter_rlm_kernel::HostRequest::tool(TOOL_NAME, TOOL_PAYLOAD, RESULT_CHAR_LIMIT)
            .expect("tool request"),
        animus_adapter_rlm_kernel::HostRequest::Subagent(child_request()),
        animus_adapter_rlm_kernel::HostRequest::context_read(CONTEXT_KEY).expect("context request"),
        animus_adapter_rlm_kernel::HostRequest::KernelClear,
    ];
    let expected_values = [
        FILE_BODY.trim(),
        "workspace edit published",
        "bounded process",
        TOOL_RESULT,
        CHILD_RESULT,
        CONTEXT_BODY.trim(),
        "kernel cleared",
    ];
    for (request, expected_value) in requests.iter().zip(expected_values) {
        let result = executor.execute_typed(request).expect("route result");
        assert_eq!(result.value(), expected_value);
        assert!(!format!("{result:?}").contains(expected_value));
    }
    assert_eq!(executor.observations().len(), requests.len());
    drop(executor);
    assert_eq!(edit.0, 1);
    assert_eq!(tool.0, 1);
    assert_eq!(child.0, 1);
    assert_eq!(kernel.0, 1);
    assert_eq!(context_resolver.calls.get(), 1);
}

#[test]
fn missing_pack_is_a_typed_construction_failure() {
    // r[verify agent_harness_runtime.rlm_host_executor.dispatch.missing]
    let route_binding = binding(RouteClass::Tool, HASH_B);
    let expected = expected_authority();
    let result = RlmHostExecutor::new(RlmHostExecutorConfig {
        bindings: BindingSet::new([route_binding]).expect("bindings"),
        expected_authority: expected.clone(),
        authority: authority(expected),
        limits: limits(),
        packs: CapabilityPacks {
            file_read: None,
            file_edit: None,
            process: None,
            tool: None,
            context: None,
            child: None,
            kernel: None,
        },
    });
    assert!(matches!(
        result,
        Err(animus_std_rlm_host::ConstructionError::PackBindingMismatch(
            RouteClass::Tool
        ))
    ));
}

#[test]
fn authority_drift_fails_before_effect() {
    // r[verify agent_harness_runtime.rlm_host_executor.dispatch.missing]
    let route_binding = binding(RouteClass::Tool, HASH_B);
    let bindings = BindingSet::new([route_binding]).expect("bindings");
    let expected = expected_authority();
    let request =
        animus_adapter_rlm_kernel::HostRequest::tool(TOOL_NAME, TOOL_PAYLOAD, RESULT_CHAR_LIMIT)
            .expect("tool request");
    for (stale_generation, cancelled, timed_out, expected_class) in [
        (true, false, false, DispatchFailureClass::Stale),
        (false, true, false, DispatchFailureClass::Cancelled),
        (false, false, true, DispatchFailureClass::TimedOut),
    ] {
        let mut tool = FakeTool(0);
        let observer = RecordingAuthority {
            expected: expected.clone(),
            stale_generation,
            cancelled,
            timed_out,
        };
        let mut executor = RlmHostExecutor::new(RlmHostExecutorConfig {
            bindings: bindings.clone(),
            expected_authority: expected.clone(),
            authority: observer,
            limits: limits(),
            packs: CapabilityPacks {
                file_read: None,
                file_edit: None,
                process: None,
                tool: Some(&mut tool),
                context: None,
                child: None,
                kernel: None,
            },
        })
        .expect("executor");
        let error = executor
            .execute_typed(&request)
            .expect_err("current recheck must fail");
        assert_eq!(error.class, expected_class);
        drop(executor);
        assert_eq!(tool.0, 0);
    }

    let mut tool = FakeTool(0);
    let mismatched_scope = RecordingAuthority {
        expected: expected.clone(),
        stale_generation: false,
        cancelled: false,
        timed_out: false,
    };
    let mut executor = RlmHostExecutor::new(RlmHostExecutorConfig {
        bindings,
        expected_authority: expected,
        authority: ScopeMismatchAuthority(mismatched_scope),
        limits: limits(),
        packs: CapabilityPacks {
            file_read: None,
            file_edit: None,
            process: None,
            tool: Some(&mut tool),
            context: None,
            child: None,
            kernel: None,
        },
    })
    .expect("executor");
    let error = executor
        .execute_typed(&request)
        .expect_err("wrong request scope must fail");
    assert_eq!(error.class, DispatchFailureClass::Stale);
    drop(executor);
    assert_eq!(tool.0, 0);
}

struct ScopeMismatchAuthority(RecordingAuthority);

impl CurrentAuthorityObserver for ScopeMismatchAuthority {
    fn observe(
        &mut self,
        route: RouteClass,
        binding: &RouteBinding,
        request_scope: &BindingIdentity,
    ) -> Result<CurrentAuthorityObservation, DispatchFailure> {
        let mut observed = self.0.observe(route, binding, request_scope)?;
        observed.request_scope_identity = BindingIdentity::new(HASH_F).expect("fixture hash");
        Ok(observed)
    }
}

#[test]
fn tool_failures_and_oversized_results_stay_typed_and_private() {
    // r[verify agent_harness_runtime.rlm_host_executor.verification]
    let route_binding = binding(RouteClass::Tool, HASH_B);
    let expected = expected_authority();
    let request =
        animus_adapter_rlm_kernel::HostRequest::tool(TOOL_NAME, TOOL_PAYLOAD, RESULT_CHAR_LIMIT)
            .expect("tool request");

    let mut failing = FailingTool;
    let mut failed_executor = RlmHostExecutor::new(RlmHostExecutorConfig {
        bindings: BindingSet::new([route_binding.clone()]).expect("bindings"),
        expected_authority: expected.clone(),
        authority: authority(expected.clone()),
        limits: limits(),
        packs: CapabilityPacks {
            file_read: None,
            file_edit: None,
            process: None,
            tool: Some(&mut failing),
            context: None,
            child: None,
            kernel: None,
        },
    })
    .expect("executor");
    assert_eq!(
        failed_executor
            .execute_typed(&request)
            .expect_err("tool failure")
            .class,
        DispatchFailureClass::OwnerFailure
    );

    let mut oversized = OversizedTool;
    let mut oversized_executor = RlmHostExecutor::new(RlmHostExecutorConfig {
        bindings: BindingSet::new([route_binding]).expect("bindings"),
        expected_authority: expected.clone(),
        authority: authority(expected),
        limits: limits(),
        packs: CapabilityPacks {
            file_read: None,
            file_edit: None,
            process: None,
            tool: Some(&mut oversized),
            context: None,
            child: None,
            kernel: None,
        },
    })
    .expect("executor");
    let error = oversized_executor
        .execute_typed(&request)
        .expect_err("oversized tool result");
    assert_eq!(error.class, DispatchFailureClass::BoundExceeded);
    assert!(!format!("{error:?}").contains("private-tool-output"));
}

#[cfg(unix)]
#[test]
fn file_read_rejects_parent_symlink_and_oversized_inputs() {
    // r[verify agent_harness_runtime.rlm_host_executor.workspace]
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().expect("temporary root");
    std::fs::write(temp.path().join(FILE_NAME), FILE_BODY).expect("fixture file");
    symlink(FILE_NAME, temp.path().join("link.txt")).expect("symlink fixture");
    std::fs::write(temp.path().join("large.txt"), "x".repeat(64)).expect("large fixture");
    for (target, expected_class) in [
        ("../outside", DispatchFailureClass::Denied),
        ("link.txt", DispatchFailureClass::Denied),
        ("large.txt", DispatchFailureClass::BoundExceeded),
    ] {
        let root = cap_std::fs::Dir::open_ambient_dir(temp.path(), cap_std::ambient_authority())
            .expect("capability root");
        let route_binding = binding(RouteClass::FileRead, HASH_B);
        let expected = expected_authority();
        let mut executor = RlmHostExecutor::new(RlmHostExecutorConfig {
            bindings: BindingSet::new([route_binding.clone()]).expect("bindings"),
            expected_authority: expected.clone(),
            authority: authority(expected),
            limits: ExecutorLimits {
                max_file_read_bytes: 8,
                ..limits()
            },
            packs: CapabilityPacks {
                file_read: Some(FileReadPack::new(root, route_binding).expect("file pack")),
                file_edit: None,
                process: None,
                tool: None,
                context: None,
                child: None,
                kernel: None,
            },
        })
        .expect("executor");
        let request = animus_adapter_rlm_kernel::HostRequest::file_read(target)
            .expect("bounded request shape");
        let error = executor
            .execute_typed(&request)
            .expect_err("unsafe read must fail");
        assert_eq!(error.class, expected_class);
    }
}

#[test]
fn unknown_process_and_oversized_context_fail_closed() {
    // r[verify agent_harness_runtime.rlm_host_executor.process.unknown]
    // r[verify agent_harness_runtime.rlm_host_executor.services.context_bound]
    let temp = tempfile::tempdir().expect("temporary root");
    let process_binding = binding(RouteClass::Shell, HASH_B);
    let process = ProcessPack::new(
        process_binding.clone(),
        process_profiles(temp.path()),
        AtomicBool::new(false),
    )
    .expect("process pack");
    let expected = expected_authority();
    let mut process_executor = RlmHostExecutor::new(RlmHostExecutorConfig {
        bindings: BindingSet::new([process_binding]).expect("bindings"),
        expected_authority: expected.clone(),
        authority: authority(expected.clone()),
        limits: limits(),
        packs: CapabilityPacks {
            file_read: None,
            file_edit: None,
            process: Some(process),
            tool: None,
            context: None,
            child: None,
            kernel: None,
        },
    })
    .expect("executor");
    let unknown = animus_adapter_rlm_kernel::HostRequest::shell("unknown", RESULT_CHAR_LIMIT)
        .expect("unknown request");
    assert_eq!(
        process_executor
            .execute_typed(&unknown)
            .expect_err("unknown profile")
            .class,
        DispatchFailureClass::Denied
    );

    let resolver = FileResolver {
        body: "private-context".repeat(16),
        calls: Cell::new(0),
    };
    let mut context = ResolverContextRoute {
        resolvers: animus_adapters::ContextResolverSet::empty().with_file(&resolver),
        policy: animus_adapters::ContextReferencePolicy::allow_local(),
    };
    let context_binding = binding(RouteClass::ContextRead, HASH_B);
    let mut context_executor = RlmHostExecutor::new(RlmHostExecutorConfig {
        bindings: BindingSet::new([context_binding]).expect("bindings"),
        expected_authority: expected.clone(),
        authority: authority(expected),
        limits: ExecutorLimits {
            max_context_bytes: 8,
            ..limits()
        },
        packs: CapabilityPacks {
            file_read: None,
            file_edit: None,
            process: None,
            tool: None,
            context: Some(&mut context),
            child: None,
            kernel: None,
        },
    })
    .expect("executor");
    let request =
        animus_adapter_rlm_kernel::HostRequest::context_read(CONTEXT_KEY).expect("context request");
    let error = context_executor
        .execute_typed(&request)
        .expect_err("oversized context must fail");
    assert_eq!(error.class, DispatchFailureClass::BoundExceeded);
    assert!(!format!("{error:?}").contains("private-context"));
}

fn all_bindings() -> Vec<RouteBinding> {
    [
        RouteClass::FileRead,
        RouteClass::FileEdit,
        RouteClass::Shell,
        RouteClass::Tool,
        RouteClass::Subagent,
        RouteClass::ContextRead,
        RouteClass::KernelClear,
    ]
    .into_iter()
    .map(|route| binding(route, HASH_B))
    .collect()
}

fn select_binding(bindings: &[RouteBinding], route: RouteClass) -> RouteBinding {
    bindings
        .iter()
        .find(|binding| binding.route() == route)
        .expect("route binding")
        .clone()
}

fn binding(route: RouteClass, identity: &str) -> RouteBinding {
    RouteBinding::new(route, identity, HASH_C, GENERATION).expect("route binding")
}

fn limits() -> ExecutorLimits {
    ExecutorLimits {
        max_file_read_bytes: FILE_BYTE_LIMIT,
        max_context_bytes: CONTEXT_BYTE_LIMIT,
        max_context_lines: CONTEXT_LINE_LIMIT,
        max_result_chars: RESULT_CHAR_LIMIT,
        max_child_status_checks: CHILD_STATUS_LIMIT,
    }
}

fn child_request() -> animus_adapter_rlm_kernel::ChildSpawnRequest {
    animus_adapter_rlm_kernel::ChildSpawnRequest::new(
        "focused-child",
        "selected context",
        1,
        1,
        512,
        5_000,
        RESULT_CHAR_LIMIT,
        false,
        false,
    )
    .expect("child request")
}

fn process_profiles(root: &Path) -> ProcessProfileSet {
    let program = std::env::var(PROCESS_FIXTURE_ENV).unwrap_or_else(|_error| {
        if cfg!(windows) {
            String::from("C:\\Windows\\System32\\cmd.exe")
        } else {
            String::from("/bin/printf")
        }
    });
    let args = if cfg!(windows) && std::env::var_os(PROCESS_FIXTURE_ENV).is_none() {
        vec![String::from("/C"), String::from("echo bounded process")]
    } else {
        vec![String::from("bounded process")]
    };
    let profile = animus_runtime::ProcessJobProfile {
        backend: Some(animus_runtime::ProcessJobBackend::Native),
        command: None,
        program: Some(program),
        args,
        cwd: Some(root.to_string_lossy().into_owned()),
        writable_paths: Vec::new(),
        env: Vec::new(),
        resources: animus_runtime::ProcessJobResourceLimits {
            timeout_seconds: Some(TIMEOUT_SECONDS),
            memory_bytes: None,
            cpu_millis: None,
            log_bytes: Some(LOG_BYTES),
        },
        notification: animus_runtime::ProcessJobNotificationPolicy::default(),
        metadata: Vec::new(),
    };
    let manifest = animus_runtime::ProcessJobProfileManifest::current(vec![
        animus_runtime::NamedProcessJobProfile {
            name: String::from("print"),
            profile,
        },
    ]);
    let source = animus_runtime::ProcessJobProfileSource::new(
        animus_runtime::ProcessJobProfileSourcePrecedence::Explicit,
        "rlm-fixture",
        manifest,
    )
    .expect("profile source");
    ProcessProfileSet {
        sources: vec![source],
        policy: animus_runtime::ProcessJobProfilePolicy {
            default_backend: animus_runtime::ProcessJobBackend::Native,
            allowed_backends: vec![animus_runtime::ProcessJobBackend::Native],
            allowed_env_prefixes: Vec::new(),
            allowed_cwd_prefixes: vec![root.to_string_lossy().into_owned()],
            allowed_writable_path_prefixes: Vec::new(),
            max_timeout_seconds: Some(TIMEOUT_SECONDS),
            max_memory_bytes: None,
            max_cpu_millis: None,
            max_log_bytes: Some(LOG_BYTES),
            policy_source: String::from("rlm-fixture-policy"),
            policy_hash: String::new(),
        }
        .with_computed_hash(),
        owner_label: String::from("rlm-host"),
    }
}

fn authority(expected: ExpectedAuthority) -> RecordingAuthority {
    RecordingAuthority {
        expected,
        stale_generation: false,
        cancelled: false,
        timed_out: false,
    }
}

fn expected_authority() -> ExpectedAuthority {
    use animus_runtime::runtime_authority_seal as seal;

    let expected_views =
        seal::RuntimeAuthorityViewClass::ALL.map(|class| seal::RuntimeAuthorityExpectedViewDraft {
            class,
            identity: HASH_A,
        });
    let observed_views =
        seal::RuntimeAuthorityViewClass::ALL.map(|class| seal::RuntimeAuthorityObservedViewDraft {
            class,
            identity: HASH_A,
            status: seal::RuntimeAuthorityObservationStatus::Current,
        });
    let operations = [HASH_A];
    let resources = [HASH_C];
    let plan = seal::prepare_runtime_authority_seal(
        seal::RuntimeAuthorityUnitDraft {
            session_id: "rlm.session",
            dispatch_id: "rlm.dispatch",
            session_lineage_identity: HASH_A,
            durable_binding_identity: HASH_A,
            authority_profile_identity: HASH_A,
            enforcement_observation_identity: HASH_A,
            operation_set_identity: HASH_A,
            adapter_cohort_identity: HASH_A,
            resource_profile_identity: HASH_A,
            resource_budget_identity: HASH_A,
            runtime_services_identity: HASH_A,
            operation_identities: &operations,
            resource_identities: &resources,
            expected_views: &expected_views,
            observed_views: &observed_views,
        },
        seal::RuntimeAuthorityReceiptDraft {
            sensitive: seal::RuntimeAuthoritySensitiveDraft::default(),
            semantic_claims: &[],
        },
    )
    .expect("authority plan");
    let unit = seal::commit_runtime_authority_seal(&plan, plan.receipt().receipt_identity.as_str())
        .expect("authority seal");
    ExpectedAuthority { unit }
}
