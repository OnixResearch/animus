use std::{env, fs, path::{Path, PathBuf}, process};

const SUCCESS: i32 = 0;
const FAILURE: i32 = 1;
const EXPECTED_ARGUMENTS: usize = 2;

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.len() != EXPECTED_ARGUMENTS {
        eprintln!("usage: check-harbor-atif-adapter <root>");
        process::exit(FAILURE);
    }
    let root = PathBuf::from(&arguments[1]);
    match validate_repository(&root) {
        Ok(()) => {
            println!("harbor ATIF adapter static check: pass");
            process::exit(SUCCESS);
        }
        Err(error) => {
            eprintln!("harbor ATIF adapter static check: {error}");
            process::exit(FAILURE);
        }
    }
}

fn validate_repository(root: &Path) -> Result<(), String> {
    let crate_root = read(root, "crates/animus-adapter-atif/src/lib.rs")?;
    let manifest = read(root, "crates/animus-adapter-atif/Cargo.toml")?;
    let model = read(root, "crates/animus-adapter-atif/src/model.rs")?;
    let codec = read(root, "crates/animus-adapter-atif/src/codec.rs")?;
    let policy = read(root, "crates/animus-adapter-atif/src/policy.rs")?;
    let validator = read(root, "crates/animus-adapter-atif/src/validate.rs")?;
    let projection = read(root, "crates/animus-adapter-atif/src/projection.rs")?;
    let receipt = read(root, "crates/animus-adapter-atif/src/receipt.rs")?;
    let tests = read(root, "crates/animus-adapter-atif/src/tests.rs")?;
    let nickel = read(root, "config/harbor-atif-profile.ncl")?;
    let generated = read(root, "config/generated/harbor-atif-profile.json")?;
    let runtime = read(root, "crates/animus-runtime/src/harbor/eval.rs")?;
    let integration = read(root, "tests/harbor-atif-integration.rs")?;
    let root_manifest = read(root, "Cargo.toml")?;
    let positive = read(root, "fixtures/harbor-atif/positive.ncl")?;
    let negative = read(root, "fixtures/harbor-atif/negative.ncl")?;

    require_all(
        &crate_root,
        &[
            "#![no_std]",
            "#![forbid(unsafe_code)]",
            "HARBOR_RELEASE",
            "527d50deb63a5d279e8c20593c18a2cbc7f61f9e",
            "ATIF-v1.7",
            "b3:78d4a45481206483c89269eeb515854552bb4185b4db41a966047d8f124cfe4f",
            "animus.harbor-atif-json-codec.v1",
        ],
        "crate cohort",
    )?;
    require_all(
        &manifest,
        &["default = []", "json-codec", "default-features = false", "optional = true"],
        "focused manifest",
    )?;
    require_all(
        &model,
        &[
            "pub struct Trajectory",
            "pub struct Agent",
            "pub struct Step",
            "pub enum Message",
            "pub struct ContentPart",
            "pub struct ToolCall",
            "pub struct ObservationResult",
            "pub struct Metrics",
            "pub struct FinalMetrics",
            "pub struct SubagentTrajectoryRef",
            "serde(deny_unknown_fields)",
        ],
        "ATIF DTOs",
    )?;
    require_all(
        &codec,
        &[
            "pub fn decode_atif_policy_json",
            "pub fn decode_atif_json",
            "pub fn encode_atif_json",
            "bytes: &[u8]",
            "serde_json::from_slice",
            "validate_trajectory",
        ],
        "caller-byte codec",
    )?;
    require_all(
        &policy,
        &[
            "pub fn validate_atif_policy",
            "pub struct AtifBounds",
            "pub struct ProjectionPolicy",
            "pub struct AtifExportDefaults",
            "ambient_resolution",
            "automatic_dataset_promotion",
            "automatic_evaluation_promotion",
        ],
        "checked policy",
    )?;
    require_all(
        &validator,
        &[
            "pub fn validate_trajectory",
            "pub fn validate_resolved_graph",
            "StepSequence",
            "ToolCorrelation",
            "NoLlm",
            "copied_context_steps",
            "ReferenceDisposition::Embedded",
            "ExternalReferencePolicy::Reject",
            "unsafe_extension_key",
            "hashable_extension",
            "animus.harbor-atif-semantic.v1",
        ],
        "semantic validator",
    )?;
    require_all(
        &projection,
        &[
            "pub fn import_validated_trajectory",
            "pub fn export_trajectory_snapshot",
            "AtifLedgerDraft",
            "AtifBatchDraft",
            "AtifLineageDraft",
            "AtifDatasetDraft",
            "AtifLossClass",
            "CopiedContext",
            "UnavailableFact",
        ],
        "loss-visible projection",
    )?;
    require_all(
        &receipt,
        &[
            "pub fn build_atif_receipt",
            "REQUIRED_ATIF_RECEIPT_FIELDS",
            "REQUIRED_ATIF_NON_CLAIMS",
            "disposition_classes",
            "animus.harbor-atif-receipt.v1",
        ],
        "safe receipts",
    )?;
    for forbidden in ["pub message", "pub reasoning", "pub tool_arguments", "pub path", "pub url"] {
        reject(&receipt, forbidden, "receipt content field")?;
    }
    require_all(
        &tests,
        &[
            "generated_nickel_policy_revalidates_in_rust",
            "valid_json_round_trips_and_projects_with_explicit_losses",
            "multimodal_no_llm_embedded_and_external_facts_remain_bounded",
            "malformed_duplicate_unknown_and_oversized_json_fail_closed",
            "semantic_validation_rejects_sequence_correlation_metrics_and_no_llm_conflicts",
            "resolved_graph_accepts_dag_and_rejects_cycle_or_missing_node",
            "unsafe_policy_and_export_without_source_facts_fail_closed",
        ],
        "positive and negative production tests",
    )?;
    require_all(
        &nickel,
        &[
            "animus.harbor-atif-profile.v1",
            "v0.18.0",
            "ATIF-v1.7",
            "source_cohort_identity",
            "max_aggregate_steps",
            "receipt_fields",
            "required_non_claims",
            "invent_terminal_state = false",
            "ambient_resolution = false",
        ],
        "Nickel profile",
    )?;
    require_all(
        &generated,
        &[
            "\"harbor_revision\": \"527d50deb63a5d279e8c20593c18a2cbc7f61f9e\"",
            "\"atif_schema\": \"ATIF-v1.7\"",
            "\"source_license\": \"Apache-2.0\"",
            "\"ambient_resolution\": false",
        ],
        "generated policy",
    )?;
    require_all(
        &runtime,
        &[
            "pub fn project_validated_atif",
            "ValidatedTrajectory",
            "AtifImportProjection",
            "does not infer or require a terminal trial outcome",
        ],
        "Harbor migration",
    )?;
    require_all(
        &integration,
        &[
            "project_validated_atif",
            "decode_atif_json",
            "import_validated_trajectory",
            "AtifError::Correlation",
        ],
        "cross-crate production integration",
    )?;
    require_all(
        &root_manifest,
        &["harbor-atif =", "harbor-atif-json =", "animus-adapter-atif"],
        "optional facade features",
    )?;
    validate_fixture_inventory(&positive, &negative)?;
    for path in [
        "docs/harbor-atif-json-adapter.md",
        "docs/harbor-evaluation-harness.md",
        "docs/contracts-index.md",
        "docs/adapter-family-crates.md",
        "docs/request-lifecycle.md",
        "docs/safety-contract.md",
        "docs/focused-verification-rails.md",
        "cairn/component-catalog.ncl",
        "cairn/runtime-facade-inventory.ncl",
        "README.md",
    ] {
        if read(root, path)?.trim().is_empty() {
            return Err(format!("{path} is empty"));
        }
    }
    validate_authority_absence(&manifest, &[&crate_root, &codec, &validator, &projection])
}

fn validate_fixture_inventory(positive: &str, negative: &str) -> Result<(), String> {
    for positive_case in [
        "text-steps",
        "multimodal-image-descriptor",
        "tool-observation-correlation",
        "finite-metrics",
        "copied-context",
        "no-llm-dispatch",
        "embedded-subagent",
        "external-subagent",
        "continuation-descriptor",
        "bounded-extra",
        "safe-import",
        "safe-export",
        "semantic-round-trip",
    ] {
        require(positive, positive_case, "positive fixtures")?;
    }
    for negative_case in [
        "malformed-json",
        "invalid-utf8",
        "oversized-json",
        "duplicate-field",
        "unknown-field",
        "unsupported-version",
        "invalid-step-id",
        "invalid-timestamp",
        "invalid-source-field",
        "invalid-content",
        "orphan-observation",
        "invalid-metrics",
        "no-llm-contradiction",
        "copied-context-promotion",
        "aggregate-bound-overflow",
        "session-id-resolution",
        "missing-embedded-target",
        "graph-cycle",
        "ambient-resolution",
        "credential-extension",
        "raw-reasoning-receipt",
        "automatic-persistence",
        "byte-round-trip-claim",
        "training-production-release-claim",
    ] {
        require(negative, negative_case, "negative fixtures")?;
    }
    Ok(())
}

fn validate_authority_absence(manifest: &str, sources: &[&str]) -> Result<(), String> {
    let manifest_lower = manifest.to_ascii_lowercase();
    for forbidden in [
        "harbor =",
        "pydantic",
        "python",
        "tokio",
        "reqwest",
        "hyper",
        "cap-std",
        "std::fs",
    ] {
        reject(&manifest_lower, forbidden, "focused dependency")?;
    }
    for source in sources {
        for forbidden in [
            "std::fs",
            "std::process",
            "std::net",
            "std::env",
            "SystemTime",
            "Command::new",
            "reqwest::",
            "pydantic",
        ] {
            reject(source, forbidden, "reusable authority")?;
        }
    }
    Ok(())
}

fn read(root: &Path, relative: &str) -> Result<String, String> {
    fs::read_to_string(root.join(relative)).map_err(|error| format!("{relative}: {error}"))
}

fn require_all(content: &str, needles: &[&str], label: &str) -> Result<(), String> {
    for needle in needles {
        require(content, needle, label)?;
    }
    Ok(())
}

fn require(content: &str, needle: &str, label: &str) -> Result<(), String> {
    if content.contains(needle) {
        Ok(())
    } else {
        Err(format!("{label} is missing `{needle}`"))
    }
}

fn reject(content: &str, needle: &str, label: &str) -> Result<(), String> {
    if content.contains(needle) {
        Err(format!("{label} contains forbidden `{needle}`"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{reject, require, require_all};

    #[test]
    fn marker_checks_accept_complete_material() {
        require_all("alpha beta gamma", &["alpha", "gamma"], "fixture")
            .expect("complete markers must pass");
        reject("pure caller bytes", "std::fs", "fixture")
            .expect("authority-free material must pass");
    }

    #[test]
    fn marker_checks_reject_missing_or_forbidden_material() {
        assert!(require("alpha", "beta", "fixture").is_err());
        assert!(reject("std::fs::read", "std::fs", "fixture").is_err());
    }
}
