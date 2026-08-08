use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const SUCCESS: u8 = 0;
const FAILURE: u8 = 1;
const SELECTED_REVISION: &str = "3b498c3d7c248c335d3e7faf12f133a18c87d632";
const REQUIRED_FILES: &[&str] = &[
    "crates/animus-tui/src/video.rs",
    "crates/animus-tui/src/videotests.rs",
    "crates/animus-tui/Cargo.toml",
    "config/terminal-video-policy.ncl",
    "config/generated/terminal-video-policy.json",
    "fixtures/terminal-video/positive.ncl",
    "fixtures/terminal-video/negative.ncl",
    "docs/terminal-video-playback.md",
    "scripts/cairn/check-terminal-video-playback",
    "flake.nix",
];
const CONTRACT_MARKERS: &[&str] = &[
    "struct TerminalVideoDescriptor",
    "struct TerminalVideoPolicy",
    "struct TerminalVideoCapabilities",
    "struct TerminalVideoState",
    "enum TerminalVideoCommand",
    "enum TerminalVideoAction",
    "fn select_terminal_video_backend",
    "fn reduce_terminal_video",
    "trait TerminalVideoFrameSource",
    "fn fetch_terminal_video_frame",
    "trait TerminalVideoBackendService",
    "fn execute_terminal_video_frame",
    "execute_terminal_image_plan",
    "fn execute_terminal_video_backend_actions",
    "fn project_terminal_video_receipt",
    "fn validate_terminal_video_evidence",
    "blake3::Hasher",
    "CleanupIncomplete",
    "RATS_IMAGE_COHORT_REVISION",
];
const TEST_MARKERS: &[&str] = &[
    "backend_selection_requires_exact_animation_row_and_preserves_fallback_order",
    "ordered_frames_advance_with_bounded_requests_and_static_replacement",
    "pause_resume_and_tick_are_explicit_and_monotonic",
    "duplicate_out_of_order_and_invalid_timing_fail_closed",
    "queue_pressure_drops_before_retention_and_enforces_drop_bound",
    "early_frame_waits_without_retaining_or_reordering",
    "cancellation_creates_exact_cleanup_and_cleanup_failure_stays_visible",
    "terminal_loss_source_failure_layout_eviction_and_shutdown_stop_requests",
    "source_finish_requires_all_frames_and_completes_cleanup",
    "injected_frame_source_rechecks_bounds_identity_order_and_timeout",
    "injected_shell_transmits_static_frame_then_calls_exact_animation_operation",
    "partial_terminal_write_fails_before_video_backend_and_attempts_image_cleanup",
    "backend_action_shell_is_injected_bounded_and_fail_closed",
    "fallback_and_cancellation_race_are_explicit",
    "malformed_descriptor_policy_frame_and_identity_fail_closed",
    "receipt_is_deterministic_metadata_only_and_leaks_fail_closed",
];
const POLICY_MARKERS: &[&str] = &[
    "max_frame_bytes",
    "max_buffered_bytes",
    "max_queue_frames",
    "max_frame_pixels",
    "max_frame_rate_millihertz",
    "max_duration_ticks",
    "max_dropped_frames",
    "max_actions",
    "max_lag_ticks",
    "max_frame_deadline_ticks",
    "max_cleanup_obligations",
    "require_injected_frame_source = true",
    "require_injected_terminal_backend = true",
    "require_explicit_monotonic_ticks = true",
    "host-admitted-kitty-animation-controls",
];
const FORBIDDEN_EFFECTS: &[&str] = &[
    "std::fs",
    "std::env",
    "std::process",
    "std::net",
    "std::time::Instant",
    "std::time::SystemTime",
    "/dev/tty",
    "io::stdout",
    "Command::new",
    "TcpStream",
    "ffmpeg",
    "gstreamer",
    "rodio",
    "reqwest",
    "tokio::time",
];

#[derive(Clone)]
struct Inputs {
    contract: String,
    tests: String,
    policy: String,
    tui_manifest: String,
    root_manifest: String,
    docs: String,
    flake: String,
    other_manifests: Vec<(String, String)>,
}

fn parse_root(args: &[String]) -> Result<PathBuf, String> {
    match args {
        [] => Ok(PathBuf::from(".")),
        [flag, root] if flag == "--root" => Ok(PathBuf::from(root)),
        _ => Err("usage: check-terminal-video-playback [--root PATH]".to_owned()),
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
        contract: read_text(root, "crates/animus-tui/src/video.rs")?,
        tests: read_text(root, "crates/animus-tui/src/videotests.rs")?,
        policy: read_text(root, "config/terminal-video-policy.ncl")?,
        tui_manifest: read_text(root, "crates/animus-tui/Cargo.toml")?,
        root_manifest: read_text(root, "Cargo.toml")?,
        docs: read_text(root, "docs/terminal-video-playback.md")?,
        flake: read_text(root, "flake.nix")?,
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
    require_markers("terminal video contract", &inputs.contract, &CONTRACT_MARKERS)?;
    require_markers("terminal video tests", &inputs.tests, &TEST_MARKERS)?;
    require_markers("terminal video policy", &inputs.policy, &POLICY_MARKERS)?;
    reject_markers("terminal video contract", &inputs.contract, &FORBIDDEN_EFFECTS)?;
    require_markers(
        "TUI feature",
        &inputs.tui_manifest,
        &["terminal-video = [\"terminal-image\"]", SELECTED_REVISION],
    )?;
    require_markers(
        "root feature",
        &inputs.root_manifest,
        &["terminal-video = [\"terminal-image\", \"animus-tui/terminal-video\"]"],
    )?;
    require_markers(
        "Nix rail",
        &inputs.flake,
        &["terminal-video-playback", "check-terminal-video-playback"],
    )?;
    require_markers(
        "documentation",
        &inputs.docs,
        &[
            SELECTED_REVISION,
            "operation-specific",
            "decoded frames",
            "does not prove",
        ],
    )?;
    for (path, manifest) in &inputs.other_manifests {
        if manifest.contains("terminal-video") || manifest.contains("rat-image") {
            return Err(format!("terminal video dependency leaked outside animus-tui: {path}"));
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
            println!("terminal video playback static check: pass");
            ExitCode::from(SUCCESS)
        }
        Err(error) => {
            eprintln!("terminal video playback static check: fail: {error}");
            ExitCode::from(FAILURE)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Inputs, SELECTED_REVISION, validate};

    fn complete() -> Inputs {
        Inputs {
            contract: "struct TerminalVideoDescriptor struct TerminalVideoPolicy struct TerminalVideoCapabilities struct TerminalVideoState enum TerminalVideoCommand enum TerminalVideoAction fn select_terminal_video_backend fn reduce_terminal_video trait TerminalVideoFrameSource fn fetch_terminal_video_frame trait TerminalVideoBackendService fn execute_terminal_video_frame execute_terminal_image_plan fn execute_terminal_video_backend_actions fn project_terminal_video_receipt fn validate_terminal_video_evidence blake3::Hasher CleanupIncomplete RATS_IMAGE_COHORT_REVISION".to_owned(),
            tests: "backend_selection_requires_exact_animation_row_and_preserves_fallback_order ordered_frames_advance_with_bounded_requests_and_static_replacement pause_resume_and_tick_are_explicit_and_monotonic duplicate_out_of_order_and_invalid_timing_fail_closed queue_pressure_drops_before_retention_and_enforces_drop_bound early_frame_waits_without_retaining_or_reordering cancellation_creates_exact_cleanup_and_cleanup_failure_stays_visible terminal_loss_source_failure_layout_eviction_and_shutdown_stop_requests source_finish_requires_all_frames_and_completes_cleanup injected_frame_source_rechecks_bounds_identity_order_and_timeout injected_shell_transmits_static_frame_then_calls_exact_animation_operation partial_terminal_write_fails_before_video_backend_and_attempts_image_cleanup backend_action_shell_is_injected_bounded_and_fail_closed fallback_and_cancellation_race_are_explicit malformed_descriptor_policy_frame_and_identity_fail_closed receipt_is_deterministic_metadata_only_and_leaks_fail_closed".to_owned(),
            policy: "max_frame_bytes max_buffered_bytes max_queue_frames max_frame_pixels max_frame_rate_millihertz max_duration_ticks max_dropped_frames max_actions max_lag_ticks max_frame_deadline_ticks max_cleanup_obligations require_injected_frame_source = true require_injected_terminal_backend = true require_explicit_monotonic_ticks = true host-admitted-kitty-animation-controls".to_owned(),
            tui_manifest: format!("terminal-video = [\"terminal-image\"] {SELECTED_REVISION}"),
            root_manifest: "terminal-video = [\"terminal-image\", \"animus-tui/terminal-video\"]".to_owned(),
            docs: format!("{SELECTED_REVISION} operation-specific decoded frames does not prove"),
            flake: "terminal-video-playback check-terminal-video-playback".to_owned(),
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
        effectful.contract.push_str(" std::time::Instant");
        assert!(validate(&effectful).is_err());

        let mut leaked = complete();
        leaked.other_manifests.push((
            "crates/animus-runtime/Cargo.toml".to_owned(),
            "terminal-video".to_owned(),
        ));
        assert!(validate(&leaked).is_err());
    }
}
