use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

// r[impl agent_harness_runtime.remora_adapters.verification]
const SUCCESS: i32 = 0;
const FAILURE: i32 = 1;
const EXPECTED_ARGUMENTS: usize = 2;
const CHANGE_REQUEST_FILES: &[&str] = &[
    "crates/animus-adapter-change-request/src/lib.rs",
    "crates/animus-adapter-change-request/src/model.rs",
    "crates/animus-adapter-change-request/src/planning.rs",
    "crates/animus-adapter-change-request/src/ports.rs",
    "crates/animus-adapter-change-request/src/receipt.rs",
];
const MAINTENANCE_FILES: &[&str] = &[
    "crates/animus-adapter-maintenance/src/lib.rs",
    "crates/animus-adapter-maintenance/src/model.rs",
    "crates/animus-adapter-maintenance/src/planning.rs",
    "crates/animus-adapter-maintenance/src/ports.rs",
    "crates/animus-adapter-maintenance/src/receipt.rs",
];
const PORT_MARKERS: &[&str] = &[
    "pub trait ChangeRequestSourcePort",
    "pub trait ReviewAnalysisPort",
    "pub trait ChangeRequestPublicationPort",
    "pub trait ReviewPublicationPort",
    "pub trait DependencyInventoryPort",
    "pub trait UpdateCandidatePort",
    "pub trait SecurityAdvisoryPort",
    "pub trait WorkspaceUpdatePort",
    "pub trait ValidationPort",
];
const CORE_MARKERS: &[&str] = &[
    "pub fn plan_code_review",
    "pub fn plan_update_run",
    "pub fn validate_binding",
    "pub fn validate_fetch_observation",
    "pub fn validate_inventory_observation",
    "pub fn validate_workspace_observation",
    "pub fn validate_review_publication",
    "pub fn build_receipt",
    "ServiceBlocked",
    "side: Option<ReviewSide>",
    "REMORA_SOURCE_COHORT_IDENTITY",
    "REMORA_SOURCE_COHORT",
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
    "tokio", "reqwest", "hyper", "serde", "git2", "gix", "octocrab", "sqlx", "rusqlite", "cron",
];

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.len() != EXPECTED_ARGUMENTS {
        eprintln!("usage: check-remora-adapter-families <root>");
        process::exit(FAILURE);
    }
    let root = PathBuf::from(&arguments[1]);
    match validate_repository(&root) {
        Ok(()) => {
            println!("Remora adapter families static check: pass");
            process::exit(SUCCESS);
        }
        Err(error) => {
            eprintln!("Remora adapter families static check: {error}");
            process::exit(FAILURE);
        }
    }
}

fn validate_repository(root: &Path) -> Result<(), String> {
    let change_request_sources = read_many(root, CHANGE_REQUEST_FILES)?;
    let maintenance_sources = read_many(root, MAINTENANCE_FILES)?;
    let change_request_manifest = read(root, "crates/animus-adapter-change-request/Cargo.toml")?;
    let maintenance_manifest = read(root, "crates/animus-adapter-maintenance/Cargo.toml")?;
    let root_manifest = read(root, "Cargo.toml")?;
    let adapter_facade = read(root, "crates/animus-adapters/src/lib.rs")?;
    let root_facade = read(root, "src/facades/host.rs")?;

    validate_source_shapes(SourceShapes {
        change_request_sources: &change_request_sources,
        maintenance_sources: &maintenance_sources,
        change_request_manifest: &change_request_manifest,
        maintenance_manifest: &maintenance_manifest,
        root_manifest: &root_manifest,
        adapter_facade: &adapter_facade,
        root_facade: &root_facade,
    })?;

    let docs = read(root, "docs/remora-adapter-families.md")?;
    for marker in [
        "git@github.com:OnixResearch/remora.git",
        "e3c857c857f4aa170d8a5b6286aad5855db541be",
        "MIT OR Apache-2.0",
        "b3:a084a8a06727b1fae59377d34754f224065351e550caadf4e12c0bbcdcc26e42",
        "crates/remora-core/src/domain.rs",
        "nine separate ports",
        "does not prove host behavior",
    ] {
        require(&docs, marker, "Remora adapter documentation")?;
    }

    for (path, marker) in [
        (
            ".cairn/component-catalog.ncl",
            "adapters.change-request-contracts",
        ),
        (
            ".cairn/component-catalog.ncl",
            "adapters.dependency-maintenance",
        ),
        ("docs/contracts-index.md", "family:change-request-adapters"),
        (
            "docs/contracts-index.md",
            "family:dependency-maintenance-adapters",
        ),
        (
            "docs/request-lifecycle.md",
            "Dependency-maintenance adapter family",
        ),
        (
            "docs/safety-contract.md",
            "Remora-derived maintenance boundary",
        ),
        (
            "docs/focused-verification-rails.md",
            "Remora-derived adapter families",
        ),
        ("README.md", "docs/remora-adapter-families.md"),
    ] {
        require(&read(root, path)?, marker, path)?;
    }

    let change_request_tests = read(root, "crates/animus-adapter-change-request/src/tests.rs")?;
    for marker in [
        "wrong_side",
        "ServiceBlocked",
        "source_and_publication_observations_bind_exact_subjects",
        "RawContent",
        "ProviderDelivery",
    ] {
        require(&change_request_tests, marker, "change-request tests")?;
    }
    let maintenance_tests = read(root, "crates/animus-adapter-maintenance/src/tests.rs")?;
    for marker in [
        "tampered_group",
        "ServiceBlocked",
        "unsafe_profiles_fail_closed",
        "RawContent",
        "UpdateCorrectness",
    ] {
        require(&maintenance_tests, marker, "maintenance tests")?;
    }
    Ok(())
}

struct SourceShapes<'a> {
    change_request_sources: &'a [String],
    maintenance_sources: &'a [String],
    change_request_manifest: &'a str,
    maintenance_manifest: &'a str,
    root_manifest: &'a str,
    adapter_facade: &'a str,
    root_facade: &'a str,
}

fn validate_source_shapes(source: SourceShapes<'_>) -> Result<(), String> {
    let change_request = source.change_request_sources.join("\n");
    let maintenance = source.maintenance_sources.join("\n");
    let all = format!("{change_request}\n{maintenance}");
    require(&change_request, "#![no_std]", "change-request crate")?;
    require(&maintenance, "#![no_std]", "maintenance crate")?;
    require(
        &change_request,
        "#![forbid(unsafe_code)]",
        "change-request crate",
    )?;
    require(&maintenance, "#![forbid(unsafe_code)]", "maintenance crate")?;
    for marker in PORT_MARKERS {
        require(&all, marker, "separate maintenance ports")?;
    }
    for marker in CORE_MARKERS {
        require(&all, marker, "adapter functional core")?;
    }
    for forbidden in FORBIDDEN_SOURCE_TOKENS {
        if all.contains(forbidden) {
            return Err(format!(
                "adapter source contains prohibited authority token {forbidden}"
            ));
        }
    }
    for (owner, manifest) in [
        ("change-request manifest", source.change_request_manifest),
        ("maintenance manifest", source.maintenance_manifest),
    ] {
        let normalized = manifest.to_ascii_lowercase();
        for dependency in FORBIDDEN_DEPENDENCIES {
            if normalized.contains(dependency) {
                return Err(format!(
                    "{owner} contains prohibited dependency {dependency}"
                ));
            }
        }
    }
    if source
        .change_request_manifest
        .contains("animus-adapter-maintenance")
    {
        return Err(String::from(
            "change-request manifest depends on dependency maintenance",
        ));
    }
    require(
        source.maintenance_manifest,
        "animus-adapter-change-request",
        "maintenance change-request dependency",
    )?;
    for marker in [
        "change-request-adapters =",
        "maintenance-adapters =",
        "animus-adapters/change-request",
        "animus-adapters/maintenance",
    ] {
        require(source.root_manifest, marker, "root facade features")?;
    }
    for marker in [
        "pub mod change_request",
        "pub mod maintenance",
        "pub use animus_adapter_change_request::*",
        "pub use animus_adapter_maintenance::*",
    ] {
        require(
            source.adapter_facade,
            marker,
            "adapter compatibility facade",
        )?;
    }
    for marker in [
        "change_request_adapters",
        "maintenance_adapters",
        "feature = \"change-request-adapters\"",
        "feature = \"maintenance-adapters\"",
    ] {
        require(source.root_facade, marker, "root compatibility facade")?;
    }
    Ok(())
}

fn read_many(root: &Path, paths: &[&str]) -> Result<Vec<String>, String> {
    paths
        .iter()
        .map(|path| read(root, path))
        .collect::<Result<Vec<_>, _>>()
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

    fn complete_sources() -> (Vec<String>, Vec<String>) {
        let mut source = String::from("#![no_std]\n#![forbid(unsafe_code)]\n");
        for marker in PORT_MARKERS.iter().chain(CORE_MARKERS) {
            source.push_str(marker);
            source.push('\n');
        }
        (vec![source.clone()], vec![source])
    }

    fn complete_shapes<'a>(
        change_request_sources: &'a [String],
        maintenance_sources: &'a [String],
    ) -> SourceShapes<'a> {
        SourceShapes {
            change_request_sources,
            maintenance_sources,
            change_request_manifest: "[dependencies]\nblake3 = {}",
            maintenance_manifest:
                "[dependencies]\nanimus-adapter-change-request = {}\nblake3 = {}",
            root_manifest: "change-request-adapters = [animus-adapters/change-request]\nmaintenance-adapters = [animus-adapters/maintenance]",
            adapter_facade: "pub mod change_request pub mod maintenance pub use animus_adapter_change_request::* pub use animus_adapter_maintenance::*",
            root_facade: "feature = \"change-request-adapters\" feature = \"maintenance-adapters\" change_request_adapters maintenance_adapters",
        }
    }

    #[test]
    fn accepts_complete_authority_free_shapes() {
        let (change_request_sources, maintenance_sources) = complete_sources();
        assert_eq!(
            validate_source_shapes(complete_shapes(
                &change_request_sources,
                &maintenance_sources,
            )),
            Ok(())
        );
    }

    #[test]
    fn rejects_host_authority_and_reversed_dependency() {
        let (mut change_request_sources, maintenance_sources) = complete_sources();
        change_request_sources[0].push_str("use std::process;");
        let mut shapes = complete_shapes(&change_request_sources, &maintenance_sources);
        shapes.change_request_manifest =
            "[dependencies]\nanimus-adapter-maintenance = {}\ntokio = {}";
        assert!(validate_source_shapes(shapes).is_err());
    }
}
