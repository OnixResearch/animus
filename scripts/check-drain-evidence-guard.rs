#!/usr/bin/env -S nix shell "github:nix-community/fenix?rev=092bd452904e749efa39907aa4a20a42678ac31e#minimal.toolchain" nixpkgs#gcc -c cargo -q -Zscript

//! Prevent loss of ignored `.pi/` evidence during linked-worktree teardown.
//! r[impl agent_harness_runtime.drain_evidence_guard.teardown]
//! r[impl agent_harness_runtime.drain_evidence_guard.fail_closed]
//! r[impl agent_harness_runtime.drain_evidence_guard.evidence]

use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const PROGRAM_NAME: &str = "check-drain-evidence-guard.rs";
const HELP_FLAG: &str = "--help";
const SELF_TEST_FLAG: &str = "--self-test";
const SHOW_EMPTY_FLAG: &str = "--show-empty";
const WORKTREE_FLAG: &str = "--worktree";
const EVIDENCE_DIR: &str = ".pi";
const GIT_MARKER: &str = ".git";
const GIT_WORKTREE_PREFIX: &str = "worktree ";
const MAX_SCAN_DEPTH: usize = 64;
const MAX_REPORTED_EVIDENCE_ENTRIES: usize = 4_096;
const MAX_TARGET_LABEL_CHARS: usize = 80;
const WORKTREE_OPTION_WIDTH: usize = 2;
const SUCCESS_EXIT_CODE: u8 = 0;
const FAILURE_EXIT_CODE: u8 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    worktrees: Vec<PathBuf>,
    show_empty: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorktreeKind {
    Primary,
    Linked,
    Missing,
    NotDirectory,
    MissingGitMarker,
    UnsupportedGitMarker,
    ObservationFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EvidenceState {
    NotScanned,
    Absent,
    Empty,
    ExternalSymlink,
    InternalSymlink,
    Entries,
    MalformedRoot,
    ObservationFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Observation {
    label: String,
    worktree_kind: WorktreeKind,
    evidence_state: EvidenceState,
    entry_count: usize,
    count_truncated: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Verdict {
    Pass,
    Persistent,
    Block,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BoundedCount {
    count: usize,
    truncated: bool,
}

fn main() -> ExitCode {
    match run(env::args_os().collect()) {
        Ok(true) => ExitCode::from(SUCCESS_EXIT_CODE),
        Ok(false) => ExitCode::from(FAILURE_EXIT_CODE),
        Err(message) => {
            eprintln!("drain evidence guard: verdict=ERROR reason={message}");
            ExitCode::from(FAILURE_EXIT_CODE)
        }
    }
}

fn run(raw_args: Vec<OsString>) -> Result<bool, String> {
    let args = raw_args.into_iter().skip(1).collect::<Vec<_>>();
    if args.as_slice() == [OsString::from(HELP_FLAG)] {
        print_usage();
        return Ok(true);
    }
    if args.as_slice() == [OsString::from(SELF_TEST_FLAG)] {
        run_self_tests()?;
        println!("drain evidence guard self-test: PASS");
        return Ok(true);
    }

    let config = parse_options(args)?;
    let targets = if config.worktrees.is_empty() {
        registered_worktrees()?
    } else {
        config.worktrees
    };
    if targets.is_empty() {
        return Err("no-worktrees".to_string());
    }

    let observations = targets
        .iter()
        .map(|path| observe_worktree(path))
        .collect::<Vec<_>>();
    let blocked = observations
        .iter()
        .filter(|observation| decide(observation) == Verdict::Block)
        .count();
    let linked = observations
        .iter()
        .filter(|observation| observation.worktree_kind == WorktreeKind::Linked)
        .count();
    let primary = observations
        .iter()
        .filter(|observation| observation.worktree_kind == WorktreeKind::Primary)
        .count();

    for observation in &observations {
        let verdict = decide(observation);
        if config.show_empty || verdict == Verdict::Block {
            println!("{}", render_observation(observation, verdict));
        }
    }

    let verdict = if blocked == 0 { "PASS" } else { "BLOCK" };
    println!(
        "drain evidence guard: verdict={verdict} scanned={} linked={linked} primary={primary} blocked={blocked}",
        observations.len()
    );
    Ok(blocked == 0)
}

fn print_usage() {
    println!(
        "{PROGRAM_NAME} [{WORKTREE_FLAG} PATH]... [{SHOW_EMPTY_FLAG}]\n\
         Scan linked Git worktrees for ignored .pi evidence before teardown.\n\
         With no explicit worktree, scan every registered Git worktree.\n\
         {SELF_TEST_FLAG} runs credential-free positive and negative tests."
    );
}

fn parse_options(args: Vec<OsString>) -> Result<Config, String> {
    let mut worktrees = Vec::new();
    let mut show_empty = false;
    let mut index = 0;
    while index < args.len() {
        match args[index].to_string_lossy().as_ref() {
            WORKTREE_FLAG => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| "missing-worktree-value".to_string())?;
                if value.is_empty() {
                    return Err("empty-worktree-value".to_string());
                }
                worktrees.push(PathBuf::from(value));
                index += WORKTREE_OPTION_WIDTH;
            }
            SHOW_EMPTY_FLAG => {
                show_empty = true;
                index += 1;
            }
            HELP_FLAG | SELF_TEST_FLAG => return Err("exclusive-option-misuse".to_string()),
            _ => return Err("unknown-option".to_string()),
        }
    }
    Ok(Config {
        worktrees,
        show_empty,
    })
}

fn registered_worktrees() -> Result<Vec<PathBuf>, String> {
    let output = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .output()
        .map_err(|_| "git-worktree-list-unavailable".to_string())?;
    if !output.status.success() {
        return Err("git-worktree-list-failed".to_string());
    }
    let text =
        String::from_utf8(output.stdout).map_err(|_| "git-worktree-list-non-utf8".to_string())?;
    let paths = text
        .lines()
        .filter_map(|line| line.strip_prefix(GIT_WORKTREE_PREFIX))
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    Ok(paths)
}

fn observe_worktree(path: &Path) -> Observation {
    let label = safe_target_label(path);
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return observation(label, WorktreeKind::Missing, EvidenceState::NotScanned)
        }
        Err(_) => {
            return observation(
                label,
                WorktreeKind::ObservationFailed,
                EvidenceState::NotScanned,
            )
        }
    };
    if !metadata.is_dir() {
        return observation(label, WorktreeKind::NotDirectory, EvidenceState::NotScanned);
    }

    let marker_path = path.join(GIT_MARKER);
    let marker = match fs::symlink_metadata(&marker_path) {
        Ok(marker) => marker,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return observation(
                label,
                WorktreeKind::MissingGitMarker,
                EvidenceState::NotScanned,
            )
        }
        Err(_) => {
            return observation(
                label,
                WorktreeKind::ObservationFailed,
                EvidenceState::NotScanned,
            )
        }
    };
    if marker.is_dir() {
        return observation(label, WorktreeKind::Primary, EvidenceState::NotScanned);
    }
    if !marker.is_file() {
        return observation(
            label,
            WorktreeKind::UnsupportedGitMarker,
            EvidenceState::NotScanned,
        );
    }

    observe_linked_evidence(path, label)
}

fn observe_linked_evidence(worktree: &Path, label: String) -> Observation {
    let evidence_path = worktree.join(EVIDENCE_DIR);
    let metadata = match fs::symlink_metadata(&evidence_path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return observation(label, WorktreeKind::Linked, EvidenceState::Absent)
        }
        Err(_) => {
            return observation(
                label,
                WorktreeKind::Linked,
                EvidenceState::ObservationFailed,
            )
        }
    };

    if metadata.file_type().is_symlink() {
        let evidence_target = fs::canonicalize(&evidence_path);
        let worktree_target = fs::canonicalize(worktree);
        return match (evidence_target, worktree_target) {
            (Ok(evidence_target), Ok(worktree_target))
                if evidence_target.starts_with(&worktree_target) =>
            {
                observation(label, WorktreeKind::Linked, EvidenceState::InternalSymlink)
            }
            (Ok(_), Ok(_)) => {
                observation(label, WorktreeKind::Linked, EvidenceState::ExternalSymlink)
            }
            _ => observation(
                label,
                WorktreeKind::Linked,
                EvidenceState::ObservationFailed,
            ),
        };
    }
    if !metadata.is_dir() {
        return observation(label, WorktreeKind::Linked, EvidenceState::MalformedRoot);
    }

    match count_entries(&evidence_path) {
        Ok(BoundedCount { count: 0, .. }) => {
            observation(label, WorktreeKind::Linked, EvidenceState::Empty)
        }
        Ok(count) => Observation {
            label,
            worktree_kind: WorktreeKind::Linked,
            evidence_state: EvidenceState::Entries,
            entry_count: count.count,
            count_truncated: count.truncated,
        },
        Err(()) => observation(
            label,
            WorktreeKind::Linked,
            EvidenceState::ObservationFailed,
        ),
    }
}

fn observation(
    label: String,
    worktree_kind: WorktreeKind,
    evidence_state: EvidenceState,
) -> Observation {
    Observation {
        label,
        worktree_kind,
        evidence_state,
        entry_count: 0,
        count_truncated: false,
    }
}

fn count_entries(root: &Path) -> Result<BoundedCount, ()> {
    let mut count = BoundedCount {
        count: 0,
        truncated: false,
    };
    let mut pending = vec![(root.to_path_buf(), 0_usize)];
    while let Some((directory, depth)) = pending.pop() {
        if depth > MAX_SCAN_DEPTH {
            return Err(());
        }
        let entries = fs::read_dir(&directory).map_err(|_| ())?;
        for entry in entries {
            let entry = entry.map_err(|_| ())?;
            if !record_entry(&mut count) {
                return Ok(count);
            }
            let file_type = entry.file_type().map_err(|_| ())?;
            if file_type.is_dir() {
                pending.push((entry.path(), depth + 1));
            }
        }
    }
    Ok(count)
}

fn record_entry(count: &mut BoundedCount) -> bool {
    if count.count == MAX_REPORTED_EVIDENCE_ENTRIES {
        count.truncated = true;
        false
    } else {
        count.count += 1;
        true
    }
}

fn decide(observation: &Observation) -> Verdict {
    match observation.worktree_kind {
        WorktreeKind::Primary => Verdict::Persistent,
        WorktreeKind::Linked => match observation.evidence_state {
            EvidenceState::Absent | EvidenceState::Empty | EvidenceState::ExternalSymlink => {
                Verdict::Pass
            }
            EvidenceState::NotScanned
            | EvidenceState::InternalSymlink
            | EvidenceState::Entries
            | EvidenceState::MalformedRoot
            | EvidenceState::ObservationFailed => Verdict::Block,
        },
        WorktreeKind::Missing
        | WorktreeKind::NotDirectory
        | WorktreeKind::MissingGitMarker
        | WorktreeKind::UnsupportedGitMarker
        | WorktreeKind::ObservationFailed => Verdict::Block,
    }
}

fn render_observation(observation: &Observation, verdict: Verdict) -> String {
    format!(
        "target={} worktree={} evidence={} count={} truncated={} verdict={}",
        observation.label,
        worktree_kind_label(observation.worktree_kind),
        evidence_state_label(observation.evidence_state),
        observation.entry_count,
        observation.count_truncated,
        verdict_label(verdict)
    )
}

fn worktree_kind_label(kind: WorktreeKind) -> &'static str {
    match kind {
        WorktreeKind::Primary => "primary",
        WorktreeKind::Linked => "linked",
        WorktreeKind::Missing => "missing",
        WorktreeKind::NotDirectory => "not-directory",
        WorktreeKind::MissingGitMarker => "missing-git-marker",
        WorktreeKind::UnsupportedGitMarker => "unsupported-git-marker",
        WorktreeKind::ObservationFailed => "observation-failed",
    }
}

fn evidence_state_label(state: EvidenceState) -> &'static str {
    match state {
        EvidenceState::NotScanned => "not-scanned",
        EvidenceState::Absent => "absent",
        EvidenceState::Empty => "empty",
        EvidenceState::ExternalSymlink => "external-symlink",
        EvidenceState::InternalSymlink => "internal-symlink",
        EvidenceState::Entries => "entries",
        EvidenceState::MalformedRoot => "malformed-root",
        EvidenceState::ObservationFailed => "observation-failed",
    }
}

fn verdict_label(verdict: Verdict) -> &'static str {
    match verdict {
        Verdict::Pass => "PASS",
        Verdict::Persistent => "PERSISTENT",
        Verdict::Block => "BLOCK",
    }
}

fn safe_target_label(path: &Path) -> String {
    let candidate = path
        .file_name()
        .unwrap_or(path.as_os_str())
        .to_string_lossy();
    let mut label = candidate
        .chars()
        .take(MAX_TARGET_LABEL_CHARS)
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-') {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    if label.is_empty() {
        label.push_str("unnamed-target");
    }
    label
}

// r[verify agent_harness_runtime.drain_evidence_guard.verification]
fn run_self_tests() -> Result<(), String> {
    verify_pure_classification()?;
    verify_option_parsing()?;
    verify_filesystem_cases()?;
    verify_redacted_output()?;
    Ok(())
}

fn verify_pure_classification() -> Result<(), String> {
    let label = "fixture".to_string();
    let safe_states = [
        EvidenceState::Absent,
        EvidenceState::Empty,
        EvidenceState::ExternalSymlink,
    ];
    for state in safe_states {
        let candidate = observation(label.clone(), WorktreeKind::Linked, state);
        require(
            decide(&candidate) == Verdict::Pass,
            "safe linked state must pass",
        )?;
    }

    let blocking_states = [
        EvidenceState::NotScanned,
        EvidenceState::InternalSymlink,
        EvidenceState::Entries,
        EvidenceState::MalformedRoot,
        EvidenceState::ObservationFailed,
    ];
    for state in blocking_states {
        let candidate = observation(label.clone(), WorktreeKind::Linked, state);
        require(
            decide(&candidate) == Verdict::Block,
            "unsafe linked state must block",
        )?;
    }

    let invalid_kinds = [
        WorktreeKind::Missing,
        WorktreeKind::NotDirectory,
        WorktreeKind::MissingGitMarker,
        WorktreeKind::UnsupportedGitMarker,
        WorktreeKind::ObservationFailed,
    ];
    for kind in invalid_kinds {
        let candidate = observation(label.clone(), kind, EvidenceState::NotScanned);
        require(
            decide(&candidate) == Verdict::Block,
            "invalid target kind must block",
        )?;
    }
    let primary = observation(label, WorktreeKind::Primary, EvidenceState::NotScanned);
    require(
        decide(&primary) == Verdict::Persistent,
        "primary target must remain persistent",
    )?;

    let mut bounded_count = BoundedCount {
        count: MAX_REPORTED_EVIDENCE_ENTRIES,
        truncated: false,
    };
    require(
        !record_entry(&mut bounded_count) && bounded_count.truncated,
        "entry counting must stop and report truncation at the bound",
    )
}

fn verify_option_parsing() -> Result<(), String> {
    let valid = parse_options(vec![
        OsString::from(WORKTREE_FLAG),
        OsString::from("fixture"),
        OsString::from(SHOW_EMPTY_FLAG),
    ])?;
    require(valid.show_empty, "show-empty option must parse")?;
    require(valid.worktrees.len() == 1, "one worktree must parse")?;
    require(
        parse_options(vec![OsString::from("--unknown")]).is_err(),
        "unknown option must fail",
    )?;
    require(
        parse_options(vec![OsString::from(WORKTREE_FLAG)]).is_err(),
        "missing worktree value must fail",
    )?;
    require(
        parse_options(vec![OsString::from(WORKTREE_FLAG), OsString::new()]).is_err(),
        "empty worktree value must fail",
    )
}

fn verify_filesystem_cases() -> Result<(), String> {
    let root = env::temp_dir().join(format!(
        "animus-drain-evidence-guard-self-test-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).map_err(|_| "self-test-root-create-failed".to_string())?;

    let result = verify_filesystem_cases_in(&root);
    let cleanup_result = fs::remove_dir_all(&root);
    result?;
    cleanup_result.map_err(|_| "self-test-root-cleanup-failed".to_string())?;
    Ok(())
}

fn verify_filesystem_cases_in(root: &Path) -> Result<(), String> {
    let primary = root.join("primary");
    fs::create_dir_all(primary.join(GIT_MARKER)).map_err(|_| "primary-create-failed")?;
    fs::create_dir_all(primary.join(EVIDENCE_DIR)).map_err(|_| "primary-pi-create-failed")?;
    fs::write(
        primary.join(EVIDENCE_DIR).join("secret-name"),
        "secret-body",
    )
    .map_err(|_| "primary-evidence-write-failed")?;
    require(
        decide(&observe_worktree(&primary)) == Verdict::Persistent,
        "primary worktree must be persistent",
    )?;

    let empty = root.join("empty-linked");
    create_linked_fixture(&empty)?;
    require(
        decide(&observe_worktree(&empty)) == Verdict::Pass,
        "linked worktree without .pi must pass",
    )?;
    fs::create_dir(empty.join(EVIDENCE_DIR)).map_err(|_| "empty-pi-create-failed")?;
    require(
        decide(&observe_worktree(&empty)) == Verdict::Pass,
        "linked worktree with empty .pi must pass",
    )?;

    let live = root.join("live-linked");
    create_linked_fixture(&live)?;
    fs::create_dir_all(live.join(EVIDENCE_DIR).join("private").join("nested"))
        .map_err(|_| "live-pi-create-failed")?;
    fs::write(
        live.join(EVIDENCE_DIR)
            .join("private")
            .join("nested")
            .join("wid-secret"),
        "credential-secret-body",
    )
    .map_err(|_| "live-evidence-write-failed")?;
    let live_observation = observe_worktree(&live);
    require(
        decide(&live_observation) == Verdict::Block,
        "live evidence must block",
    )?;
    require(
        live_observation.entry_count > 0,
        "live evidence must have a positive bounded count",
    )?;

    let missing = root.join("missing");
    require(
        decide(&observe_worktree(&missing)) == Verdict::Block,
        "missing explicit target must block",
    )?;

    let malformed = root.join("malformed");
    fs::create_dir_all(&malformed).map_err(|_| "malformed-create-failed")?;
    require(
        decide(&observe_worktree(&malformed)) == Verdict::Block,
        "missing git marker must block",
    )?;

    #[cfg(unix)]
    verify_symlink_cases(root)?;
    Ok(())
}

#[cfg(unix)]
fn verify_symlink_cases(root: &Path) -> Result<(), String> {
    let persistent = root.join("persistent-evidence");
    fs::create_dir_all(&persistent).map_err(|_| "persistent-create-failed")?;
    let external = root.join("external-symlink");
    create_linked_fixture(&external)?;
    std::os::unix::fs::symlink(&persistent, external.join(EVIDENCE_DIR))
        .map_err(|_| "external-symlink-create-failed")?;
    require(
        decide(&observe_worktree(&external)) == Verdict::Pass,
        "external evidence symlink must pass",
    )?;

    let internal = root.join("internal-symlink");
    create_linked_fixture(&internal)?;
    let internal_target = internal.join("removable-evidence");
    fs::create_dir_all(&internal_target).map_err(|_| "internal-target-create-failed")?;
    std::os::unix::fs::symlink(&internal_target, internal.join(EVIDENCE_DIR))
        .map_err(|_| "internal-symlink-create-failed")?;
    require(
        decide(&observe_worktree(&internal)) == Verdict::Block,
        "internal evidence symlink must block",
    )
}

fn create_linked_fixture(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|_| "linked-fixture-create-failed")?;
    fs::write(path.join(GIT_MARKER), "gitdir: fixture")
        .map_err(|_| "linked-marker-write-failed".to_string())
}

fn verify_redacted_output() -> Result<(), String> {
    let observation = Observation {
        label: safe_target_label(Path::new("unsafe target/value")),
        worktree_kind: WorktreeKind::Linked,
        evidence_state: EvidenceState::Entries,
        entry_count: MAX_REPORTED_EVIDENCE_ENTRIES,
        count_truncated: true,
    };
    let output = render_observation(&observation, decide(&observation));
    require(!output.contains('/'), "output must omit path separators")?;
    require(
        !output.contains("wid-secret"),
        "output must omit evidence names",
    )?;
    require(
        !output.contains("credential-secret-body"),
        "output must omit evidence contents",
    )?;
    require(
        output.contains("verdict=BLOCK"),
        "output must retain verdict",
    )?;
    require(
        output.contains("truncated=true"),
        "output must retain bounded-count state",
    )
}

fn require(condition: bool, message: &str) -> Result<(), String> {
    if condition {
        Ok(())
    } else {
        Err(format!("self-test-failed:{message}"))
    }
}
