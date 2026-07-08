use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ARG_ROOT: &str = "--root";
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;
const TUI_MANIFEST: &str = "crates/animus-tui/Cargo.toml";
const TUI_LIB: &str = "crates/animus-tui/src/lib.rs";
const TUI_BRIDGE: &str = "crates/animus-tui/src/bridge.rs";
const TUI_RUNNER: &str = "crates/animus-tui/src/runner.rs";
const TUI_RENDER_MODEL: &str = "crates/animus-tui/src/view/model.rs";
const TUI_RENDER_TEXT: &str = "crates/animus-tui/src/view/text.rs";
const RATS_WIDGET_PACKAGE: &str = "package = \"rat-widgets\"";
const RATS_KEYMAP_PACKAGE: &str = "package = \"rat-keymap\"";
const RATS_GIT: &str = "git = \"ssh://git@github.com/brittonr/rats.git\"";
const REV_PREFIX: &str = "rev = \"";
const FLOATING_BRANCH: &str = "branch =";
const FLOATING_TAG: &str = "tag =";
const DIRECT_CROSSTERM_DEPENDENCY: &str = "crossterm =";
const REQUIRED_SUBWAYRAT_DEPENDENCIES: usize = 2;
const PUBLIC_RAT_KEYMAP_REEXPORT: &str = "pub use rat_keymap";
const PUBLIC_RATS_REEXPORT: &str = "pub use rats";
const TOOL_HOST_EVENT_EXPORT: &str = "ToolHostEvent";
const TOOL_RENDER_MODEL_EXPORT: &str = "ToolRenderModel";
const GUARDED_TOOL_EXPORT: &str = "run_guarded_tool";
const ISSUE_TUI_MANIFEST_MISSING: &str = "RATATUI_TOOL_HOST_TUI_MANIFEST_MISSING";
const ISSUE_RATS_NOT_PINNED: &str = "RATATUI_TOOL_HOST_RATS_NOT_PINNED";
const ISSUE_RATS_FLOATING: &str = "RATATUI_TOOL_HOST_RATS_FLOATING";
const ISSUE_TUI_DIRECT_TERMINAL_DEPENDENCY: &str = "RATATUI_TOOL_HOST_DIRECT_TERMINAL_DEPENDENCY";
const ISSUE_REUSABLE_DEPENDENCY_LEAK: &str = "RATATUI_TOOL_HOST_REUSABLE_DEPENDENCY_LEAK";
const ISSUE_PUBLIC_API_LEAK: &str = "RATATUI_TOOL_HOST_PUBLIC_API_LEAK";
const ISSUE_BRIDGE_TERMINAL_LEAK: &str = "RATATUI_TOOL_HOST_BRIDGE_TERMINAL_LEAK";
const ISSUE_TUI_EXPORT_MISSING: &str = "RATATUI_TOOL_HOST_EXPORT_MISSING";

const REUSABLE_MANIFESTS: &[&str] = &[
    "crates/animus-core/Cargo.toml",
    "crates/animus-adapters/Cargo.toml",
    "crates/animus-steel/Cargo.toml",
    "crates/animus-runtime/Cargo.toml",
    "crates/animus-std/Cargo.toml",
];

const REUSABLE_LIBS: &[&str] = &[
    "crates/animus-core/src/lib.rs",
    "crates/animus-adapters/src/lib.rs",
    "crates/animus-steel/src/lib.rs",
    "crates/animus-runtime/src/lib.rs",
    "crates/animus-std/src/lib.rs",
];

const REUSABLE_DEPENDENCY_TOKENS: &[&str] = &[
    "rat-widgets",
    "rat-keymap",
    "github.com/brittonr/rats",
    "ratatui =",
    "crossterm =",
    "rat-keymap =",
    "rats =",
];

const PUBLIC_API_TOKENS: &[&str] = &[
    "pub use rat_keymap",
    "pub use rats",
    "pub type Rats",
    "pub struct Rats",
    "pub enum Rats",
    "pub trait Rats",
    "rat_keymap::",
    "ratatui::",
    "crossterm::",
];

const BRIDGE_TERMINAL_TOKENS: &[&str] = &["rat_keymap::", "ratatui::", "crossterm::", "rats::"];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    root: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Issue {
    code: &'static str,
    path: String,
    detail: String,
}

fn main() {
    match run() {
        Ok(()) => std::process::exit(EXIT_SUCCESS),
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(EXIT_FAILURE);
        }
    }
}

fn run() -> Result<(), String> {
    let config = parse_args(env::args())?;
    let mut issues = Vec::new();
    issues.extend(validate_tui_manifest(&config.root));
    issues.extend(validate_reusable_manifests(&config.root));
    issues.extend(validate_reusable_public_exports(&config.root));
    issues.extend(validate_tui_exports(&config.root));
    issues.extend(validate_bridge_neutrality(&config.root));
    issues.sort();

    for issue in &issues {
        println!("ERROR {} {} -- {}", issue.code, issue.path, issue.detail);
    }

    if issues.is_empty() {
        println!("ratatui tool-host rail: ok");
        Ok(())
    } else {
        Err(format!(
            "ratatui tool-host rail found {} issue(s)",
            issues.len()
        ))
    }
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
    let _program = args.next();
    let mut root = PathBuf::from(".");

    while let Some(argument) = args.next() {
        match argument.as_str() {
            ARG_ROOT => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_ROOT} requires a path"))?;
                root = PathBuf::from(value);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    Ok(Config { root })
}

fn validate_tui_manifest(root: &Path) -> Vec<Issue> {
    let Ok(text) = read_required(root, TUI_MANIFEST) else {
        return vec![issue(
            ISSUE_TUI_MANIFEST_MISSING,
            TUI_MANIFEST,
            "missing animus-tui manifest",
        )];
    };
    let mut issues = Vec::new();
    let has_subwayrat_packages =
        text.contains(RATS_WIDGET_PACKAGE) && text.contains(RATS_KEYMAP_PACKAGE);
    let has_pinned_subwayrat_sources = text.matches(RATS_GIT).count() >= REQUIRED_SUBWAYRAT_DEPENDENCIES
        && text.matches(REV_PREFIX).count() >= REQUIRED_SUBWAYRAT_DEPENDENCIES;
    if !(has_subwayrat_packages && has_pinned_subwayrat_sources) {
        issues.push(issue(
            ISSUE_RATS_NOT_PINNED,
            TUI_MANIFEST,
            "animus-tui must depend on Subwayrat rat-widgets and rat-keymap through the SSH git URL with explicit revs",
        ));
    }
    if text.contains(DIRECT_CROSSTERM_DEPENDENCY) {
        issues.push(issue(
            ISSUE_TUI_DIRECT_TERMINAL_DEPENDENCY,
            TUI_MANIFEST,
            "animus-tui key handling must go through Subwayrat rat-keymap instead of a direct Crossterm dependency",
        ));
    }
    if text.contains(FLOATING_BRANCH) || text.contains(FLOATING_TAG) {
        issues.push(issue(
            ISSUE_RATS_FLOATING,
            TUI_MANIFEST,
            "Rats dependency must not use floating branch or tag selectors",
        ));
    }
    issues
}

fn validate_reusable_manifests(root: &Path) -> Vec<Issue> {
    let mut issues = Vec::new();
    for manifest in REUSABLE_MANIFESTS {
        match read_required(root, manifest) {
            Ok(text) => {
                for token in REUSABLE_DEPENDENCY_TOKENS {
                    if text.contains(token) {
                        issues.push(issue(
                            ISSUE_REUSABLE_DEPENDENCY_LEAK,
                            manifest,
                            &format!("reusable manifest contains terminal dependency token: {token}"),
                        ));
                    }
                }
            }
            Err(error) => issues.push(issue(ISSUE_REUSABLE_DEPENDENCY_LEAK, manifest, &error)),
        }
    }
    issues
}

fn validate_reusable_public_exports(root: &Path) -> Vec<Issue> {
    let mut issues = Vec::new();
    for lib in REUSABLE_LIBS {
        match read_required(root, lib) {
            Ok(text) => {
                for token in PUBLIC_API_TOKENS {
                    if text.contains(token) {
                        issues.push(issue(
                            ISSUE_PUBLIC_API_LEAK,
                            lib,
                            &format!("reusable public lib exposes terminal token: {token}"),
                        ));
                    }
                }
            }
            Err(error) => issues.push(issue(ISSUE_PUBLIC_API_LEAK, lib, &error)),
        }
    }
    issues
}

fn validate_tui_exports(root: &Path) -> Vec<Issue> {
    let Ok(text) = read_required(root, TUI_LIB) else {
        return vec![issue(
            ISSUE_TUI_EXPORT_MISSING,
            TUI_LIB,
            "missing animus-tui lib exports",
        )];
    };
    let required_exports = [
        PUBLIC_RAT_KEYMAP_REEXPORT,
        PUBLIC_RATS_REEXPORT,
        TOOL_HOST_EVENT_EXPORT,
        TOOL_RENDER_MODEL_EXPORT,
        GUARDED_TOOL_EXPORT,
    ];
    let mut issues = Vec::new();
    for export in required_exports {
        if !text.contains(export) {
            issues.push(issue(
                ISSUE_TUI_EXPORT_MISSING,
                TUI_LIB,
                &format!("animus-tui missing expected host-only export: {export}"),
            ));
        }
    }
    issues
}

fn validate_bridge_neutrality(root: &Path) -> Vec<Issue> {
    let mut issues = Vec::new();
    for relative in [TUI_BRIDGE, TUI_RUNNER, TUI_RENDER_MODEL, TUI_RENDER_TEXT] {
        match read_required(root, relative) {
            Ok(text) => {
                for token in BRIDGE_TERMINAL_TOKENS {
                    if text.contains(token) {
                        issues.push(issue(
                            ISSUE_BRIDGE_TERMINAL_LEAK,
                            relative,
                            &format!("UI-neutral bridge/reducer contains terminal token: {token}"),
                        ));
                    }
                }
            }
            Err(error) => issues.push(issue(ISSUE_BRIDGE_TERMINAL_LEAK, relative, &error)),
        }
    }
    issues
}

fn read_required(root: &Path, relative: &str) -> Result<String, String> {
    let path = root.join(relative);
    fs::read_to_string(&path).map_err(|error| format!("failed to read {relative}: {error}"))
}

fn issue(code: &'static str, path: &str, detail: &str) -> Issue {
    Issue {
        code,
        path: path.to_string(),
        detail: detail.to_string(),
    }
}
