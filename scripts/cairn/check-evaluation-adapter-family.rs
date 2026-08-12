use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

const SUCCESS: i32 = 0;
const FAILURE: i32 = 1;
const EXPECTED_ARGUMENTS: usize = 2;
const CRATE_FILES: &[&str] = &[
    "crates/animus-adapter-eval/src/lib.rs",
    "crates/animus-adapter-eval/src/catalog.rs",
    "crates/animus-adapter-eval/src/binding.rs",
    "crates/animus-adapter-eval/src/ports.rs",
    "crates/animus-adapter-eval/src/gateway.rs",
    "crates/animus-adapter-eval/src/receipt.rs",
];
const PORT_MARKERS: &[&str] = &[
    "pub trait TaskSourcePort",
    "pub trait AgentExecutionPort",
    "pub trait VerifierExecutionPort",
    "pub trait ArtifactCollectPort",
    "pub trait ArtifactRestorePort",
    "pub trait ModelCallCapturePort",
    "pub trait ResultAppendPort",
    "pub trait ResultReplacePort",
    "pub trait ResultReopenPort",
];
const FORBIDDEN_SOURCE_TOKENS: &[&str] = &[
    "use std::",
    "std::fs",
    "std::process",
    "std::net",
    "std::env",
    "SystemTime",
    "Instant::now",
    "TcpStream",
    "Command::",
];
const FORBIDDEN_DEPENDENCIES: &[&str] = &[
    "tokio",
    "reqwest",
    "hyper",
    "serde",
    "python",
    "harbor",
    "docker",
    "kubernetes",
    "pydantic",
];

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.len() != EXPECTED_ARGUMENTS {
        eprintln!("usage: check-evaluation-adapter-family <root>");
        process::exit(FAILURE);
    }
    let root = PathBuf::from(&arguments[1]);
    match validate_repository(&root) {
        Ok(()) => {
            println!("evaluation adapter family static check: pass");
            process::exit(SUCCESS);
        }
        Err(error) => {
            eprintln!("evaluation adapter family static check: {error}");
            process::exit(FAILURE);
        }
    }
}

fn validate_repository(root: &Path) -> Result<(), String> {
    let crate_sources = CRATE_FILES
        .iter()
        .map(|path| read(root, path))
        .collect::<Result<Vec<_>, _>>()?;
    let manifest = read(root, "crates/animus-adapter-eval/Cargo.toml")?;
    let harbor = read(root, "crates/animus-adapters/src/harbor.rs")?;
    let root_manifest = read(root, "Cargo.toml")?;
    validate_source_shapes(SourceShapes {
        crate_sources: &crate_sources,
        manifest: &manifest,
        harbor: &harbor,
        root_manifest: &root_manifest,
    })?;

    let docs = read(root, "docs/evaluation-adapter-family.md")?;
    for marker in [
        "PrimeIntellect-ai/verifiers",
        "facd1eab5ae08491dbab9ae51a85aac73b90415e61030b551d1f7f3f3ffc5918",
        "verifiers/v1/dialects/base.py",
        "verifiers/v1/interception/server.py",
        "verifiers/v1/state.py",
        "not prove host behavior",
    ] {
        require(&docs, marker, "adapter documentation")?;
    }
    let negative_fixture = read(root, "fixtures/evaluation-adapter/negative.ncl")?;
    for marker in [
        "missing-service",
        "denied-capability",
        "wrong-plan-task-attempt",
        "shared-verifier-environment",
        "artifact-manifest-widening",
        "early-terminal-event",
        "content-only-retry",
        "storage-evidence-mismatch",
        "raw-provider-body",
        "release-overclaim",
    ] {
        require(&negative_fixture, marker, "negative fixture")?;
    }
    require(
        &read(root, "fixtures/evaluation-adapter/positive.ncl")?,
        "all-separate-ports",
        "positive fixture",
    )?;
    for (path, marker) in [
        (
            ".cairn/component-catalog.ncl",
            "adapters.native-evaluation-services",
        ),
        (
            ".cairn/runtime-facade-inventory.ncl",
            "evaluation_adapters_record \"animus-adapters::evaluation\"",
        ),
        (
            "docs/contracts-index.md",
            "family:native-evaluation-adapters",
        ),
        (
            "docs/request-lifecycle.md",
            "Native evaluation adapter family",
        ),
        (
            "docs/safety-contract.md",
            "native evaluation adapter family",
        ),
    ] {
        require(&read(root, path)?, marker, path)?;
    }
    Ok(())
}

struct SourceShapes<'a> {
    crate_sources: &'a [String],
    manifest: &'a str,
    harbor: &'a str,
    root_manifest: &'a str,
}

fn validate_source_shapes(source: SourceShapes<'_>) -> Result<(), String> {
    let all = source.crate_sources.join("\n");
    require(&all, "#![no_std]", "adapter crate")?;
    require(&all, "#![forbid(unsafe_code)]", "adapter crate")?;
    for marker in PORT_MARKERS {
        require(&all, marker, "separate evaluation ports")?;
    }
    for marker in [
        "pub fn validate_service_catalog",
        "pub fn admit_service",
        "pub fn build_service_request",
        "pub fn validate_service_observation",
        "pub fn project_model_call",
        "pub fn admit_terminal_after_commit",
        "pub fn admit_identical_request_coalescing",
        "pub fn project_adapter_receipt",
        "VERIFIERS_SOURCE_COHORT_IDENTITY",
    ] {
        require(&all, marker, "adapter core")?;
    }
    for forbidden in FORBIDDEN_SOURCE_TOKENS {
        if all.contains(forbidden) {
            return Err(format!("adapter source contains prohibited authority token {forbidden}"));
        }
    }
    let normalized_manifest = source.manifest.to_ascii_lowercase();
    for dependency in FORBIDDEN_DEPENDENCIES {
        if normalized_manifest.contains(dependency) {
            return Err(format!("adapter manifest contains prohibited dependency {dependency}"));
        }
    }
    require(
        source.manifest,
        "animus-eval-core",
        "adapter manifest native-core dependency",
    )?;
    for marker in [
        "pub fn bridge_harbor_request",
        "pub fn bridge_harbor_observation",
        "harbor_artifact_manifest_identity",
        "HarborCompatibilityError::IdentityLoss",
    ] {
        require(source.harbor, marker, "Harbor compatibility")?;
    }
    for marker in [
        "evaluation-adapters =",
        "animus-adapters/evaluation",
        "animus-adapter-eval",
    ] {
        require(source.root_manifest, marker, "root feature facade")?;
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
        let mut source = String::from("#![no_std]\n#![forbid(unsafe_code)]\n");
        for marker in PORT_MARKERS {
            source.push_str(marker);
            source.push('\n');
        }
        for marker in [
            "pub fn validate_service_catalog",
            "pub fn admit_service",
            "pub fn build_service_request",
            "pub fn validate_service_observation",
            "pub fn project_model_call",
            "pub fn admit_terminal_after_commit",
            "pub fn admit_identical_request_coalescing",
            "pub fn project_adapter_receipt",
            "VERIFIERS_SOURCE_COHORT_IDENTITY",
        ] {
            source.push_str(marker);
            source.push('\n');
        }
        vec![source]
    }

    #[test]
    fn accepts_complete_authority_free_shapes() {
        let sources = complete_sources();
        let result = validate_source_shapes(SourceShapes {
            crate_sources: &sources,
            manifest: "[dependencies]\nanimus-eval-core = {}",
            harbor: "pub fn bridge_harbor_request pub fn bridge_harbor_observation harbor_artifact_manifest_identity HarborCompatibilityError::IdentityLoss",
            root_manifest: "evaluation-adapters = [animus-adapters/evaluation, animus-adapter-eval]",
        });
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn rejects_host_authority_and_missing_bridge() {
        let mut sources = complete_sources();
        sources[0].push_str("use std::process;");
        let result = validate_source_shapes(SourceShapes {
            crate_sources: &sources,
            manifest: "[dependencies]\ntokio = {}\nanimus-eval-core = {}",
            harbor: "not bridged",
            root_manifest: "not featured",
        });
        assert!(result.is_err());
    }
}
