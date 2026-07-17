use std::path::{Path, PathBuf};

use animus_adapters::{SafeHash, SandboxDescriptor, SandboxDescriptorDraft};
use animus_std::sandlock_host::{
    SandlockAuthorizationBinding, SandlockAuthorizationBindingDraft, SandlockAuthorizationDecision,
    SandlockCapabilityRootGrant, SandlockHostPolicy, SandlockHostPolicyDraft, SandlockRootAccess,
    SandlockRootBinding, sandlock_command_hash,
};

pub(crate) const FIXTURE_BINARY: &str = env!("CARGO_BIN_EXE_animus-sandlock-live-fixture");
pub(crate) const FORBIDDEN_FILE: &str = "/etc/passwd";
pub(crate) const CREATED_FILE: &str = "sandlock-live-created.txt";
pub(crate) const DEFAULT_PROCESS_LIMIT: u32 = 32;
pub(crate) const DENIAL_PROCESS_LIMIT: u32 = 1;
const LABEL_BINARY: &str = "fixture-binary";
const LABEL_STORE: &str = "nix-store";
const LABEL_WORKSPACE: &str = "workspace";
const RELATIVE_CURRENT: &str = ".";
const RELATIVE_STORE: &str = "store";
const NIX_ROOT: &str = "/nix";
const POLICY_ID: &str = "sandlock:live-linux";
const TIMEOUT_MILLIS: u64 = 15_000;
const MAX_OUTPUT_BYTES: u64 = 65_536;
const MAX_MEMORY_BYTES: u64 = 134_217_728;
const MAX_OPEN_FILES: u32 = 64;
const MAX_CPU_PERCENT: u8 = 100;

pub(crate) struct LiveRoot {
    parent: PathBuf,
    pub(crate) workspace: PathBuf,
}

impl LiveRoot {
    pub(crate) fn new() -> Self {
        let parent = std::env::temp_dir()
            .canonicalize()
            .expect("temporary root should resolve");
        let workspace = parent.join(format!("animus-sandlock-live-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(workspace.as_path());
        std::fs::create_dir(workspace.as_path()).expect("live workspace should be created");
        Self { parent, workspace }
    }
}

impl Drop for LiveRoot {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(self.workspace.as_path());
    }
}

pub(crate) struct LiveFixture {
    pub(crate) descriptor: SandboxDescriptor,
    pub(crate) policy: SandlockHostPolicy,
    pub(crate) authorization: SandlockAuthorizationBinding,
    pub(crate) action_hash: SafeHash,
    pub(crate) subject_hash: SafeHash,
    pub(crate) target_hash: SafeHash,
    pub(crate) command_hash: SafeHash,
    pub(crate) bindings: Vec<SandlockRootBinding>,
    pub(crate) grants: Vec<SandlockCapabilityRootGrant>,
}

impl LiveFixture {
    pub(crate) fn new(live_root: &LiveRoot, command: &[&str], process_limit: u32) -> Self {
        let (bindings, grants) = live_roots(live_root);
        let descriptor = live_descriptor();
        let policy = live_policy(process_limit);
        let action_hash = SafeHash::from_blake3_bytes(b"live-action");
        let subject_hash = SafeHash::from_blake3_bytes(b"live-subject");
        let target_hash = SafeHash::from_blake3_bytes(b"live-target");
        let command_hash = sandlock_command_hash(command).expect("live command should hash");
        let authorization = live_authorization(LiveAuthorizationInput {
            descriptor: &descriptor,
            policy: &policy,
            action_hash: &action_hash,
            subject_hash: &subject_hash,
            target_hash: &target_hash,
            command_hash: &command_hash,
        });
        Self {
            descriptor,
            policy,
            authorization,
            action_hash,
            subject_hash,
            target_hash,
            command_hash,
            bindings,
            grants,
        }
    }
}

fn live_roots(
    live_root: &LiveRoot,
) -> (Vec<SandlockRootBinding>, Vec<SandlockCapabilityRootGrant>) {
    let executable = Path::new(FIXTURE_BINARY)
        .canonicalize()
        .expect("fixture executable should resolve");
    let executable_parent = executable
        .parent()
        .expect("fixture executable should have a parent")
        .to_path_buf();
    let workspace_name = live_root
        .workspace
        .file_name()
        .and_then(|value| value.to_str())
        .expect("workspace name should be UTF-8");
    let binary_grant = SafeHash::from_blake3_bytes(b"live-binary-grant");
    let store_grant = SafeHash::from_blake3_bytes(b"live-store-grant");
    let workspace_grant = SafeHash::from_blake3_bytes(b"live-workspace-grant");
    let bindings = vec![
        root_binding(
            LABEL_BINARY,
            binary_grant.clone(),
            RELATIVE_CURRENT,
            SandlockRootAccess::ReadOnly,
        ),
        root_binding(
            LABEL_STORE,
            store_grant.clone(),
            RELATIVE_STORE,
            SandlockRootAccess::ReadOnly,
        ),
        root_binding(
            LABEL_WORKSPACE,
            workspace_grant.clone(),
            workspace_name,
            SandlockRootAccess::ReadWrite,
        ),
    ];
    let grants = vec![
        root_grant(LABEL_BINARY, binary_grant, executable_parent),
        root_grant(LABEL_STORE, store_grant, PathBuf::from(NIX_ROOT)),
        root_grant(LABEL_WORKSPACE, workspace_grant, live_root.parent.clone()),
    ];
    (bindings, grants)
}

fn root_binding(
    label: &str,
    grant_hash: SafeHash,
    relative_path: &str,
    access: SandlockRootAccess,
) -> SandlockRootBinding {
    SandlockRootBinding::new(label, grant_hash, relative_path, access)
        .expect("live root binding should validate")
}

fn root_grant(
    label: &str,
    grant_hash: SafeHash,
    root_path: PathBuf,
) -> SandlockCapabilityRootGrant {
    SandlockCapabilityRootGrant::new(label, grant_hash, root_path)
        .expect("live capability root should validate")
}

fn live_descriptor() -> SandboxDescriptor {
    SandboxDescriptor::new(SandboxDescriptorDraft {
        descriptor_id: "sandlock:live-linux",
        filesystem_mode: "workspace_write",
        writable_roots: &[LABEL_WORKSPACE],
        workdir_mode: "cow_staged",
        cow_disposition: "dry_run",
        process_profile: "resource_limited",
        environment_policy: "minimal",
        network_policy: "deny_all",
        http_policy: "none",
        determinism_policy: "host",
        enforcement_requirement: "required",
    })
    .expect("live descriptor should validate")
}

fn live_policy(process_limit: u32) -> SandlockHostPolicy {
    SandlockHostPolicy::new(SandlockHostPolicyDraft {
        policy_id: POLICY_ID,
        timeout_millis: TIMEOUT_MILLIS,
        max_output_bytes: MAX_OUTPUT_BYTES,
        max_memory_bytes: Some(MAX_MEMORY_BYTES),
        max_processes: Some(process_limit),
        max_open_files: Some(MAX_OPEN_FILES),
        max_cpu_percent: Some(MAX_CPU_PERCENT),
        egress_allow: &[],
        http_allow: &[],
        http_deny: &[],
        deterministic_epoch_seconds: None,
        deterministic_seed: None,
        cow_workdir_root: Some(LABEL_WORKSPACE),
    })
    .expect("live policy should validate")
}

#[derive(Clone, Copy)]
struct LiveAuthorizationInput<'a> {
    descriptor: &'a SandboxDescriptor,
    policy: &'a SandlockHostPolicy,
    action_hash: &'a SafeHash,
    subject_hash: &'a SafeHash,
    target_hash: &'a SafeHash,
    command_hash: &'a SafeHash,
}

fn live_authorization(input: LiveAuthorizationInput<'_>) -> SandlockAuthorizationBinding {
    SandlockAuthorizationBinding::new(SandlockAuthorizationBindingDraft {
        authorization_hash: SafeHash::from_blake3_bytes(b"live-authorization"),
        action_hash: input.action_hash.clone(),
        subject_hash: input.subject_hash.clone(),
        target_hash: input.target_hash.clone(),
        descriptor_hash: input.descriptor.descriptor_hash().clone(),
        policy_hash: input.policy.policy_hash().clone(),
        command_hash: input.command_hash.clone(),
        decision: SandlockAuthorizationDecision::Allowed,
        current: true,
    })
}
