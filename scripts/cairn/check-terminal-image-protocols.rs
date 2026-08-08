use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const SUCCESS: u8 = 0;
const FAILURE: u8 = 1;
const SELECTED_REVISION: &str = "3b498c3d7c248c335d3e7faf12f133a18c87d632";
const OLD_REVISION: &str = "e9cb7865fe400de6220aea8bfc08ddf4f75da3d3";
const REQUIRED_FILES: [&str; 10] = [
    "crates/animus-tui/src/pixel.rs",
    "crates/animus-tui/src/pixeltests.rs",
    "crates/animus-tui/Cargo.toml",
    "config/terminal-image-policy.ncl",
    "config/generated/terminal-image-policy.json",
    "fixtures/terminal-image/positive.ncl",
    "fixtures/terminal-image/negative.ncl",
    "docs/terminal-image-protocols.md",
    "Cargo.lock",
    "flake.nix",
];
const CONTRACT_MARKERS: [&str; 15] = [
    "struct TerminalImageCapabilities",
    "struct TerminalImagePolicy",
    "enum TerminalImageOperation",
    "struct TerminalImagePlan",
    "fn parse_kitty_capability_response",
    "fn plan_terminal_image",
    "trait TerminalImageByteResolver",
    "trait TerminalImageLifecycleService",
    "fn execute_terminal_image_plan",
    "rat_image::prepare_terminal_image",
    "rat_image::write_prepared_terminal_image",
    "fn project_terminal_image_receipt",
    "fn validate_terminal_image_evidence",
    "blake3::Hasher",
    "RATS_IMAGE_COHORT_REVISION",
];
const TEST_MARKERS: [&str; 12] = [
    "kitty_capability_response_parsing_is_bounded_and_content_free",
    "kitty_plan_preserves_the_static_lifecycle_order",
    "kitty_transmission_uses_bounded_multiple_upstream_chunks",
    "alternate_protocols_admit_only_their_exact_transmit_row",
    "explicit_resolver_and_writer_execute_rats_transmit_without_ambient_terminal_access",
    "failed_write_keeps_cleanup_failure_visible",
    "malformed_safe_identities_and_metadata_fail_closed",
    "image_source_terminal_alt_text_and_lifecycle_bounds_fail_closed",
    "unsupported_destructive_operations_and_denied_fallback_fail_closed",
    "resolver_failure_and_identity_drift_do_not_write",
    "evidence_validator_rejects_terminal_and_host_content",
    "receipt_is_deterministic_and_omits_handle_alt_text_and_protocol_payload",
];
const POLICY_MARKERS: [&str; 11] = [
    "max_image_bytes",
    "max_prepared_bytes",
    "max_capability_response_bytes",
    "max_source_pixels",
    "max_terminal_cells",
    "max_chunks",
    "max_operations",
    "max_lifecycle_entries",
    "require_injected_byte_resolver = true",
    "require_injected_writer = true",
    "rat-image-transmit-only",
];
const FORBIDDEN_EFFECTS: [&str; 9] = [
    "std::fs",
    "std::env",
    "std::process",
    "std::net",
    "/dev/tty",
    "OpenOptions",
    "io::stdout",
    "Command::new",
    "TcpStream",
];

#[derive(Clone)]
struct Inputs {
    contract: String,
    tests: String,
    policy: String,
    tui_manifest: String,
    root_manifest: String,
    cargo_lock: String,
    flake: String,
    docs: String,
    other_manifests: Vec<(String, String)>,
}

fn parse_root(args: &[String]) -> Result<PathBuf, String> {
    match args {
        [] => Ok(PathBuf::from(".")),
        [flag, root] if flag == "--root" => Ok(PathBuf::from(root)),
        _ => Err("usage: check-terminal-image-protocols [--root PATH]".to_owned()),
    }
}

fn read_text(root: &Path, relative: &str) -> Result<String, String> {
    fs::read_to_string(root.join(relative))
        .map_err(|error| format!("cannot read {relative}: {error}"))
}

fn collect_other_manifests(root: &Path) -> Result<Vec<(String, String)>, String> {
    let crates = root.join("crates");
    let entries = fs::read_dir(&crates)
        .map_err(|error| format!("cannot list {}: {error}", crates.display()))?;
    let mut manifests = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|error| format!("cannot inspect crate entry: {error}"))?;
        let path = entry.path().join("Cargo.toml");
        if path.is_file() && !path.ends_with("animus-tui/Cargo.toml") {
            let text = fs::read_to_string(&path)
                .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
            manifests.push((path.display().to_string(), text));
        }
    }
    Ok(manifests)
}

fn read_inputs(root: &Path) -> Result<Inputs, String> {
    for relative in REQUIRED_FILES {
        if !root.join(relative).is_file() {
            return Err(format!("missing required file: {relative}"));
        }
    }
    Ok(Inputs {
        contract: read_text(root, "crates/animus-tui/src/pixel.rs")?,
        tests: read_text(root, "crates/animus-tui/src/pixeltests.rs")?,
        policy: read_text(root, "config/terminal-image-policy.ncl")?,
        tui_manifest: read_text(root, "crates/animus-tui/Cargo.toml")?,
        root_manifest: read_text(root, "Cargo.toml")?,
        cargo_lock: read_text(root, "Cargo.lock")?,
        flake: read_text(root, "flake.nix")?,
        docs: read_text(root, "docs/terminal-image-protocols.md")?,
        other_manifests: collect_other_manifests(root)?,
    })
}

fn require_markers(label: &str, text: &str, markers: &[&str]) -> Result<(), String> {
    let missing: Vec<&str> = markers
        .iter()
        .copied()
        .filter(|marker| !text.contains(marker))
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!("{label} misses markers: {}", missing.join(", ")))
    }
}

fn reject_markers(label: &str, text: &str, markers: &[&str]) -> Result<(), String> {
    let present: Vec<&str> = markers
        .iter()
        .copied()
        .filter(|marker| text.contains(marker))
        .collect();
    if present.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{label} contains forbidden markers: {}",
            present.join(", ")
        ))
    }
}

fn validate(inputs: &Inputs) -> Result<(), String> {
    require_markers("terminal image contract", &inputs.contract, &CONTRACT_MARKERS)?;
    require_markers("terminal image tests", &inputs.tests, &TEST_MARKERS)?;
    require_markers("terminal image policy", &inputs.policy, &POLICY_MARKERS)?;
    reject_markers("terminal image contract", &inputs.contract, &FORBIDDEN_EFFECTS)?;
    require_markers(
        "TUI manifest",
        &inputs.tui_manifest,
        &[
            "terminal-image = [\"dep:blake3\", \"dep:rat-image\"]",
            "package = \"rat-image\"",
            SELECTED_REVISION,
        ],
    )?;
    require_markers(
        "root feature",
        &inputs.root_manifest,
        &["terminal-image = [\"tui\", \"animus-tui/terminal-image\"]"],
    )?;
    require_markers(
        "Cargo lock",
        &inputs.cargo_lock,
        &["name = \"rat-image\"", SELECTED_REVISION],
    )?;
    require_markers("Nix pin", &inputs.flake, &[SELECTED_REVISION])?;
    reject_markers("TUI manifest", &inputs.tui_manifest, &[OLD_REVISION])?;
    reject_markers("Cargo lock", &inputs.cargo_lock, &[OLD_REVISION])?;
    reject_markers("Nix pin", &inputs.flake, &[OLD_REVISION])?;
    require_markers(
        "documentation",
        &inputs.docs,
        &[
            SELECTED_REVISION,
            "MIT",
            "1.88.0",
            "operation-level",
            "does not prove",
        ],
    )?;
    for (path, manifest) in &inputs.other_manifests {
        if manifest.contains("rat-image") {
            return Err(format!("rat-image dependency leaked outside animus-tui: {path}"));
        }
    }
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    let root = parse_root(args)?;
    let inputs = read_inputs(&root)?;
    validate(&inputs)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => {
            println!("terminal image protocol static check: pass");
            ExitCode::from(SUCCESS)
        }
        Err(error) => {
            eprintln!("terminal image protocol static check: fail: {error}");
            ExitCode::from(FAILURE)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Inputs, SELECTED_REVISION, validate};

    fn complete() -> Inputs {
        Inputs {
            contract: "struct TerminalImageCapabilities struct TerminalImagePolicy enum TerminalImageOperation struct TerminalImagePlan fn parse_kitty_capability_response fn plan_terminal_image trait TerminalImageByteResolver trait TerminalImageLifecycleService fn execute_terminal_image_plan rat_image::prepare_terminal_image rat_image::write_prepared_terminal_image fn project_terminal_image_receipt fn validate_terminal_image_evidence blake3::Hasher RATS_IMAGE_COHORT_REVISION".to_owned(),
            tests: "kitty_capability_response_parsing_is_bounded_and_content_free kitty_plan_preserves_the_static_lifecycle_order kitty_transmission_uses_bounded_multiple_upstream_chunks alternate_protocols_admit_only_their_exact_transmit_row explicit_resolver_and_writer_execute_rats_transmit_without_ambient_terminal_access failed_write_keeps_cleanup_failure_visible malformed_safe_identities_and_metadata_fail_closed image_source_terminal_alt_text_and_lifecycle_bounds_fail_closed unsupported_destructive_operations_and_denied_fallback_fail_closed resolver_failure_and_identity_drift_do_not_write evidence_validator_rejects_terminal_and_host_content receipt_is_deterministic_and_omits_handle_alt_text_and_protocol_payload".to_owned(),
            policy: "max_image_bytes max_prepared_bytes max_capability_response_bytes max_source_pixels max_terminal_cells max_chunks max_operations max_lifecycle_entries require_injected_byte_resolver = true require_injected_writer = true rat-image-transmit-only".to_owned(),
            tui_manifest: format!("terminal-image = [\"dep:blake3\", \"dep:rat-image\"] package = \"rat-image\" {SELECTED_REVISION}"),
            root_manifest: "terminal-image = [\"tui\", \"animus-tui/terminal-image\"]".to_owned(),
            cargo_lock: format!("name = \"rat-image\" {SELECTED_REVISION}"),
            flake: SELECTED_REVISION.to_owned(),
            docs: format!("{SELECTED_REVISION} MIT 1.88.0 operation-level does not prove"),
            other_manifests: Vec::new(),
        }
    }

    #[test]
    fn accepts_complete_isolated_surface() {
        assert_eq!(validate(&complete()), Ok(()));
    }

    #[test]
    fn rejects_effectful_core_or_dependency_leak() {
        let mut effectful = complete();
        effectful.contract.push_str(" std::env");
        assert!(validate(&effectful).is_err());

        let mut leaked = complete();
        leaked
            .other_manifests
            .push(("crates/animus-core/Cargo.toml".to_owned(), "rat-image".to_owned()));
        assert!(validate(&leaked).is_err());
    }
}
