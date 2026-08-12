use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

const SUCCESS: i32 = 0;
const FAILURE: i32 = 1;
const EXPECTED_ARGUMENTS: usize = 2;
const HOST_FILES: &[&str] = &[
    "crates/animus-std-eval/src/lib.rs",
    "crates/animus-std-eval/src/contract.rs",
    "crates/animus-std-eval/src/policy.rs",
    "crates/animus-std-eval/src/preflight.rs",
    "crates/animus-std-eval/src/gateway.rs",
    "crates/animus-std-eval/src/host.rs",
    "crates/animus-std-eval/src/receipt.rs",
    "crates/animus-std-eval/src/tests.rs",
];
const REQUIRED_PORTS: &[&str] = &[
    "TaskSourcePort",
    "AgentExecutionPort",
    "VerifierExecutionPort",
    "ArtifactCollectPort",
    "ArtifactRestorePort",
    "ModelCallCapturePort",
    "ResultAppendPort",
    "ResultReplacePort",
    "ResultReopenPort",
    "pub trait ToolPort",
    "pub trait CancellationPort",
    "pub trait DeadlinePort",
    "pub trait CleanupPort",
];
const REQUIRED_FUNCTIONS: &[&str] = &[
    "pub fn load_evaluation_host_policy",
    "pub fn parse_evaluation_host_policy",
    "pub fn validate_evaluation_host_policy",
    "pub fn validate_artifact_manifest",
    "pub fn validate_storage_target",
    "pub fn validate_gateway_invocation",
    "pub fn preflight_host",
    "pub fn execute_evaluation_host",
    "pub fn project_host_receipt",
];
const FORBIDDEN_SOURCE_TOKENS: &[&str] = &[
    "std::process",
    "std::net",
    "std::env",
    "Command::",
    "TcpListener",
    "TcpStream",
    "UdpSocket",
    "SystemTime",
    "Instant::now",
    "docker",
    "modal",
    "prime_client",
    "registry_client",
];
const FORBIDDEN_DEPENDENCIES: &[&str] = &[
    "tokio",
    "reqwest",
    "hyper",
    "axum",
    "docker",
    "bollard",
    "kube",
    "python",
    "pydantic",
    "harbor",
];

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.len() != EXPECTED_ARGUMENTS {
        eprintln!("usage: check-std-evaluation-host <root>");
        process::exit(FAILURE);
    }
    let root = PathBuf::from(&arguments[1]);
    match validate_repository(&root) {
        Ok(()) => {
            println!("standard evaluation host static check: pass");
            process::exit(SUCCESS);
        }
        Err(error) => {
            eprintln!("standard evaluation host static check: {error}");
            process::exit(FAILURE);
        }
    }
}

fn validate_repository(root: &Path) -> Result<(), String> {
    let sources = HOST_FILES
        .iter()
        .map(|path| read(root, path))
        .collect::<Result<Vec<_>, _>>()?;
    let manifest = read(root, "crates/animus-std-eval/Cargo.toml")?;
    let root_manifest = read(root, "Cargo.toml")?;
    validate_source_shapes(&sources, &manifest, &root_manifest)?;

    let docs = read(root, "docs/native-evaluation-host.md")?;
    for marker in [
        "PrimeIntellect-ai/verifiers",
        "c6336b188596db9b18cb2f0a721c9f5bc551e619342e89807e20ec45f3bb676f",
        "verifiers/v1/cli/eval/runner.py",
        "model and task-state gateway capabilities",
        "Shared-runtime judges remain advisory",
        "does not prove confinement",
    ] {
        require(&docs, marker, "host documentation")?;
    }
    let positive = read(root, "fixtures/evaluation-host/positive.ncl")?;
    for marker in [
        "all-built-in-recipes",
        "public-entrypoint-fake-services",
        "append-atomic-replace-reopen-resume",
        "metadata-only-host-receipt",
    ] {
        require(&positive, marker, "positive fixture")?;
    }
    let negative = read(root, "fixtures/evaluation-host/negative.ncl")?;
    for marker in [
        "missing-service-before-effects",
        "public-bind",
        "content-only-retry",
        "late-write-after-seal",
        "artifact-link-planting",
        "malformed-or-stale-resume",
        "failed-post-terminal-cleanup",
        "verifier-benchmark-completion-release-overclaim",
    ] {
        require(&negative, marker, "negative fixture")?;
    }
    for (path, marker) in [
        (
            "config/evaluation-host-policy.ncl",
            "evaluation_host_policy | Policy",
        ),
        (
            "config/generated/evaluation-host-policy.json",
            "animus.native-evaluation-host.v1",
        ),
        (
            ".cairn/component-catalog.ncl",
            "std.native-evaluation-host",
        ),
        (
            ".cairn/runtime-facade-inventory.ncl",
            "evaluation_host_record \"animus::evaluation_host\"",
        ),
        ("docs/contracts-index.md", "family:native-evaluation-host"),
        ("docs/request-lifecycle.md", "Native evaluation host"),
        ("docs/safety-contract.md", "native evaluation host"),
        (
            "docs/focused-verification-rails.md",
            "Standard native evaluation host",
        ),
    ] {
        require(&read(root, path)?, marker, path)?;
    }
    Ok(())
}

fn validate_source_shapes(
    sources: &[String],
    manifest: &str,
    root_manifest: &str,
) -> Result<(), String> {
    let all = sources.join("\n");
    require(&all, "#![forbid(unsafe_code)]", "host crate")?;
    require(
        &all,
        "VERIFIERS_HOST_SOURCE_COHORT_IDENTITY",
        "host source cohort",
    )?;
    require(
        &all,
        "REQUIRED_HOST_SERVICE_COUNT: usize = 9",
        "nine-service preflight",
    )?;
    require(
        &all,
        "REQUIRED_AUXILIARY_SERVICE_COUNT: usize = 4",
        "auxiliary-service preflight",
    )?;
    for marker in REQUIRED_PORTS {
        require(&all, marker, "explicit host ports")?;
    }
    for marker in REQUIRED_FUNCTIONS {
        require(&all, marker, "host lifecycle")?;
    }
    for marker in [
        "GatewayBindMode::Loopback",
        "CallerAdmitted,",
        "HostStatus::CleanupRequired",
        "HostReceiptLeakFlags",
        "RecipeKind::IsolatedAgenticJudge",
        "RecipeKind::SharedAdvisoryJudge",
    ] {
        require(&all, marker, "host fixtures and boundaries")?;
    }
    for forbidden in FORBIDDEN_SOURCE_TOKENS {
        if all.to_ascii_lowercase().contains(&forbidden.to_ascii_lowercase()) {
            return Err(format!("host source contains prohibited token {forbidden}"));
        }
    }
    let normalized_manifest = manifest.to_ascii_lowercase();
    for dependency in FORBIDDEN_DEPENDENCIES {
        if normalized_manifest.contains(dependency) {
            return Err(format!("host manifest contains prohibited dependency {dependency}"));
        }
    }
    for dependency in ["animus-adapter-eval", "animus-eval-core", "serde_json"] {
        require(manifest, dependency, "host manifest")?;
    }
    for marker in [
        "evaluation-host =",
        "dep:animus-std-eval",
        "evaluation-adapters",
    ] {
        require(root_manifest, marker, "root feature facade")?;
    }
    Ok(())
}

fn read(root: &Path, relative: &str) -> Result<String, String> {
    fs::read_to_string(root.join(relative))
        .map_err(|error| format!("cannot read {relative}: {error}"))
}

fn require(source: &str, marker: &str, owner: &str) -> Result<(), String> {
    if !source.contains(marker) {
        return Err(format!("{owner} is missing {marker}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complete_sources() -> Vec<String> {
        let mut source = String::from(
            "#![forbid(unsafe_code)]\nVERIFIERS_HOST_SOURCE_COHORT_IDENTITY\nREQUIRED_HOST_SERVICE_COUNT: usize = 9\nREQUIRED_AUXILIARY_SERVICE_COUNT: usize = 4\n",
        );
        for marker in REQUIRED_PORTS
            .iter()
            .chain(REQUIRED_FUNCTIONS.iter())
            .chain(
                [
                    "GatewayBindMode::Loopback",
                    "CallerAdmitted,",
                    "HostStatus::CleanupRequired",
                    "HostReceiptLeakFlags",
                    "RecipeKind::IsolatedAgenticJudge",
                    "RecipeKind::SharedAdvisoryJudge",
                ]
                .iter(),
            )
        {
            source.push_str(marker);
            source.push('\n');
        }
        vec![source]
    }

    #[test]
    fn accepts_explicit_bounded_host_shape() {
        let sources = complete_sources();
        assert_eq!(
            validate_source_shapes(
                &sources,
                "animus-adapter-eval animus-eval-core serde_json",
                "evaluation-host = [dep:animus-std-eval, evaluation-adapters]",
            ),
            Ok(())
        );
    }

    #[test]
    fn rejects_ambient_transport_and_missing_feature() {
        let mut sources = complete_sources();
        sources[0].push_str("TcpListener");
        assert!(
            validate_source_shapes(
                &sources,
                "animus-adapter-eval animus-eval-core serde_json tokio",
                "not-featured",
            )
            .is_err()
        );
    }
}
