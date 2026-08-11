//! Packaged local standard-input smoke for the non-default MCP client host.

use std::ffi::OsString;
use std::path::PathBuf;
use std::process;

use animus_std_mcp::{
    CredentialService, CredentialServiceError, LaunchAuthority, McpClientHost, ResolvedCredentials,
    TeardownObservation, load_generated_config,
};

const EXIT_CONFIGURATION: i32 = 2;
const EXIT_PATH: i32 = 3;
const EXIT_START: i32 = 4;
const EXIT_DISCOVERY: i32 = 5;
const EXIT_TEARDOWN: i32 = 6;
const SMOKE_GENERATION: u64 = 1;
const EMPTY_CREDENTIAL_SNAPSHOT: &str =
    "d3e2fa0ef86858e50ee0a179ba69314f69c02fc139239a9ca01a088143042284";

struct EmptyCredentials;

impl CredentialService for EmptyCredentials {
    fn resolve(
        &self,
        credential_refs: &[String],
        environment_keys: &[String],
    ) -> Result<ResolvedCredentials, CredentialServiceError> {
        if !credential_refs.is_empty() || !environment_keys.is_empty() {
            return Err(CredentialServiceError::InvalidSnapshot);
        }
        Ok(ResolvedCredentials {
            snapshot_identity: String::from(EMPTY_CREDENTIAL_SNAPSHOT),
            environment: Vec::<(OsString, OsString)>::new(),
        })
    }
}

// r[impl agent_harness_runtime.mcp_client_host.verification]
fn main() {
    let Ok(config) = load_generated_config() else {
        process::exit(EXIT_CONFIGURATION);
    };
    let Some(executable_path) = fixture_path() else {
        process::exit(EXIT_PATH);
    };
    let Ok(working_directory) = std::env::current_dir() else {
        process::exit(EXIT_PATH);
    };
    let launch = LaunchAuthority {
        executable_path,
        executable_identity: config.server().executable_identity.clone(),
        working_directory,
        working_root_identity: config.server().working_root_identity.clone(),
    };
    let host = McpClientHost::connect(config, SMOKE_GENERATION, launch, Box::new(EmptyCredentials));
    let Ok(mut host) = host else {
        process::exit(EXIT_START);
    };
    if host.catalog().tools().is_empty() || host.instructions().is_none() {
        let _teardown = host.stop();
        process::exit(EXIT_DISCOVERY);
    }
    let teardown = host.stop();
    if !matches!(
        teardown,
        TeardownObservation::Clean | TeardownObservation::Forced
    ) {
        process::exit(EXIT_TEARDOWN);
    }
    println!("animus MCP client host packaged smoke: pass");
}

fn fixture_path() -> Option<PathBuf> {
    let current = std::env::current_exe().ok()?;
    let parent = current.parent()?;
    let name = format!("animus-mcp-fixture-server{}", std::env::consts::EXE_SUFFIX);
    Some(parent.join(name))
}
