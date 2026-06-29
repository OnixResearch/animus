use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const ARG_ROOT: &str = "--root";
const ARG_ROADMAP: &str = "--roadmap";
const CAIRN_CHANGES_DIR: &str = "cairn/changes";
const CAIRN_SPECS_DIR: &str = "cairn/specs";
const FIELD_SEPARATOR: char = '|';
const CSV_SEPARATOR: char = ',';
const REQUIREMENT_START_MARKER: &str = "r[";
const REQUIREMENT_END_MARKER: char = ']';
const MARKDOWN_EXTENSION: &str = "md";
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;
const TIER_FIELD_COUNT: usize = 4;
const CHANGE_FIELD_COUNT: usize = 6;
const OVERLAP_FIELD_COUNT: usize = 4;
const ACTIVE_FIELD_COUNT: usize = 2;
const ACCEPTED_FIELD_COUNT: usize = 2;
const RECORD_KIND_TIER: &str = "tier";
const RECORD_KIND_CHANGE: &str = "change";
const RECORD_KIND_OVERLAP: &str = "overlap";
const RECORD_KIND_ACTIVE: &str = "active";
const RECORD_KIND_ACCEPTED: &str = "accepted";
const ROLE_FOUNDATIONAL: &str = "foundational";
const ROLE_CORE_RUNTIME: &str = "core-runtime";
const ROLE_HOST_ADAPTER: &str = "host-adapter";
const ROLE_OPTIONAL_REFERENCE: &str = "optional-reference";
const ROLE_PRODUCT_SURFACE: &str = "product-surface";
const ROLE_EVALUATION_ONLY: &str = "evaluation-only";
const ISSUE_MALFORMED_RECORD: &str = "ROADMAP_MALFORMED_RECORD";
const ISSUE_DUPLICATE_CHANGE_ID: &str = "ROADMAP_DUPLICATE_CHANGE_ID";
const ISSUE_DUPLICATE_TIER_ID: &str = "ROADMAP_DUPLICATE_TIER_ID";
const ISSUE_UNKNOWN_RECORD_KIND: &str = "ROADMAP_UNKNOWN_RECORD_KIND";
const ISSUE_UNKNOWN_ROLE: &str = "ROADMAP_UNKNOWN_ROLE";
const ISSUE_UNKNOWN_TIER_ID: &str = "ROADMAP_UNKNOWN_TIER_ID";
const ISSUE_UNKNOWN_CHANGE_ID: &str = "ROADMAP_UNKNOWN_CHANGE_ID";
const ISSUE_UNCLASSIFIED_ACTIVE_CHANGE: &str = "ROADMAP_UNCLASSIFIED_ACTIVE_CHANGE";
const ISSUE_UNKNOWN_PREREQUISITE_ID: &str = "ROADMAP_UNKNOWN_PREREQUISITE_ID";
const ISSUE_DEPENDENCY_CYCLE: &str = "ROADMAP_DEPENDENCY_CYCLE";
const ISSUE_UNSAFE_PRODUCT_SURFACE: &str = "ROADMAP_UNSAFE_PRODUCT_SURFACE";
const ISSUE_UNKNOWN_OVERLAP_OWNER: &str = "ROADMAP_UNKNOWN_OVERLAP_OWNER";
const ISSUE_UNKNOWN_OVERLAP_RELATED: &str = "ROADMAP_UNKNOWN_OVERLAP_RELATED";

const ALLOWED_ROLES: &[&str] = &[
    ROLE_FOUNDATIONAL,
    ROLE_CORE_RUNTIME,
    ROLE_HOST_ADAPTER,
    ROLE_OPTIONAL_REFERENCE,
    ROLE_PRODUCT_SURFACE,
    ROLE_EVALUATION_ONLY,
];

const FOUNDATIONAL_SAFETY_MARKERS: &[&str] = &[
    "wire-cairn-policy-validation",
    "dogfood-cairn-lifecycle-evidence",
    "adapt-clankers-focused-verification-rails",
    "agent_harness_runtime.cairn_policy.closeout",
    "agent_harness_runtime.cairn_dogfood.closeout",
    "agent_harness_runtime.clankers_focused_rails.closeout",
];

const RUNTIME_SAFETY_MARKERS: &[&str] = &[
    "introduce-steel-runtime-loop",
    "adapt-clankers-injected-effect-services",
    "adapt-clankers-safe-receipts-artifacts",
    "adapt-pi-runtime-event-lifecycle",
    "adapt-pi-tool-contracts",
    "agent_harness_runtime.validation",
    "agent_harness_runtime.clankers_injected_services.closeout",
    "agent_harness_runtime.clankers_safe_receipts.closeout",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    root: PathBuf,
    roadmap: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Tier {
    id: String,
    requires: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Change {
    id: String,
    role: String,
    tier: String,
    prerequisites: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Overlap {
    concept: String,
    owner: String,
    related: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Roadmap {
    tiers: Vec<Tier>,
    changes: Vec<Change>,
    overlaps: Vec<Overlap>,
    active_override: Vec<String>,
    accepted_override: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Issue {
    code: &'static str,
    subject: String,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct BlockedChange {
    change_id: String,
    prerequisite_id: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct ValidationReport {
    issues: Vec<Issue>,
    blocked: Vec<BlockedChange>,
}

fn main() {
    let result = run();
    match result {
        Ok(()) => std::process::exit(EXIT_SUCCESS),
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(EXIT_FAILURE);
        }
    }
}

fn run() -> Result<(), String> {
    let config = parse_args(env::args())?;
    let roadmap_text = fs::read_to_string(&config.roadmap)
        .map_err(|error| format!("failed to read roadmap lines: {error}"))?;
    let roadmap = parse_roadmap(&roadmap_text);
    let active_changes = active_changes_for_validation(&config.root, &roadmap)?;
    let accepted_requirements = accepted_requirements_for_validation(&config.root, &roadmap)?;
    let mut report = validate_roadmap(&roadmap, &active_changes, &accepted_requirements);
    report.issues.sort();
    report.blocked.sort();
    print_report(&roadmap, &active_changes, &report);
    if report.issues.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "roadmap validation failed with {} issue(s)",
            report.issues.len()
        ))
    }
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
    let _program = args.next();
    let mut root = None;
    let mut roadmap = None;

    while let Some(argument) = args.next() {
        match argument.as_str() {
            ARG_ROOT => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_ROOT} requires a path"))?;
                root = Some(PathBuf::from(value));
            }
            ARG_ROADMAP => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_ROADMAP} requires a path"))?;
                roadmap = Some(PathBuf::from(value));
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    let root = root.unwrap_or_else(|| PathBuf::from("."));
    let roadmap = roadmap.ok_or_else(|| format!("{ARG_ROADMAP} is required"))?;
    Ok(Config { root, roadmap })
}

fn parse_roadmap(text: &str) -> Roadmap {
    let mut roadmap = Roadmap::default();
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split(FIELD_SEPARATOR).collect();
        if fields.is_empty() {
            continue;
        }
        match fields[0] {
            RECORD_KIND_TIER if fields.len() == TIER_FIELD_COUNT => roadmap.tiers.push(Tier {
                id: fields[1].to_string(),
                requires: parse_csv(fields[2]),
            }),
            RECORD_KIND_CHANGE if fields.len() == CHANGE_FIELD_COUNT => {
                roadmap.changes.push(Change {
                    id: fields[1].to_string(),
                    role: fields[2].to_string(),
                    tier: fields[3].to_string(),
                    prerequisites: parse_csv(fields[4]),
                })
            }
            RECORD_KIND_OVERLAP if fields.len() == OVERLAP_FIELD_COUNT => {
                roadmap.overlaps.push(Overlap {
                    concept: fields[1].to_string(),
                    owner: fields[2].to_string(),
                    related: parse_csv(fields[3]),
                })
            }
            RECORD_KIND_ACTIVE if fields.len() == ACTIVE_FIELD_COUNT => {
                roadmap.active_override.push(fields[1].to_string());
            }
            RECORD_KIND_ACCEPTED if fields.len() == ACCEPTED_FIELD_COUNT => {
                roadmap.accepted_override.push(fields[1].to_string());
            }
            RECORD_KIND_TIER | RECORD_KIND_CHANGE | RECORD_KIND_OVERLAP | RECORD_KIND_ACTIVE
            | RECORD_KIND_ACCEPTED => {
                roadmap.changes.push(Change {
                    id: format!("{ISSUE_MALFORMED_RECORD}:{line}"),
                    role: String::new(),
                    tier: String::new(),
                    prerequisites: Vec::new(),
                });
            }
            _unknown => {
                roadmap.changes.push(Change {
                    id: format!("{ISSUE_UNKNOWN_RECORD_KIND}:{line}"),
                    role: String::new(),
                    tier: String::new(),
                    prerequisites: Vec::new(),
                });
            }
        }
    }
    roadmap
}

fn parse_csv(text: &str) -> Vec<String> {
    if text.trim().is_empty() {
        return Vec::new();
    }
    text.split(CSV_SEPARATOR)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn active_changes_for_validation(
    root: &Path,
    roadmap: &Roadmap,
) -> Result<BTreeSet<String>, String> {
    if !roadmap.active_override.is_empty() {
        return Ok(roadmap.active_override.iter().cloned().collect());
    }
    read_active_changes(root).map_err(|error| format!("failed to read active changes: {error}"))
}

fn accepted_requirements_for_validation(
    root: &Path,
    roadmap: &Roadmap,
) -> Result<BTreeSet<String>, String> {
    if !roadmap.accepted_override.is_empty() {
        return Ok(roadmap.accepted_override.iter().cloned().collect());
    }
    read_accepted_requirements(root)
        .map_err(|error| format!("failed to read accepted requirements: {error}"))
}

fn read_active_changes(root: &Path) -> io::Result<BTreeSet<String>> {
    let mut changes = BTreeSet::new();
    let changes_dir = root.join(CAIRN_CHANGES_DIR);
    for entry_result in fs::read_dir(changes_dir)? {
        let entry = entry_result?;
        if entry.file_type()?.is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                changes.insert(name.to_string());
            }
        }
    }
    Ok(changes)
}

fn read_accepted_requirements(root: &Path) -> io::Result<BTreeSet<String>> {
    let specs_dir = root.join(CAIRN_SPECS_DIR);
    let mut files = Vec::new();
    collect_markdown_files(&specs_dir, &mut files)?;
    let mut ids = BTreeSet::new();
    for file in files {
        let text = fs::read_to_string(file)?;
        ids.extend(extract_requirement_ids(&text));
    }
    Ok(ids)
}

fn collect_markdown_files(root: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if !root.exists() {
        return Ok(());
    }

    let mut pending = VecDeque::from([root.to_path_buf()]);
    while let Some(dir) = pending.pop_front() {
        for entry_result in fs::read_dir(dir)? {
            let entry = entry_result?;
            let path = entry.path();
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                pending.push_back(path);
            } else if file_type.is_file()
                && path.extension() == Some(OsStr::new(MARKDOWN_EXTENSION))
            {
                files.push(path);
            }
        }
    }

    files.sort();
    Ok(())
}

fn extract_requirement_ids(text: &str) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    let mut remaining = text;
    while let Some(start_index) = remaining.find(REQUIREMENT_START_MARKER) {
        let after_start = &remaining[start_index + REQUIREMENT_START_MARKER.len()..];
        let Some(end_index) = after_start.find(REQUIREMENT_END_MARKER) else {
            break;
        };
        let id = &after_start[..end_index];
        if !id.trim().is_empty() {
            ids.insert(id.trim().to_string());
        }
        remaining = &after_start[end_index..];
    }
    ids
}

fn validate_roadmap(
    roadmap: &Roadmap,
    active_changes: &BTreeSet<String>,
    accepted_requirements: &BTreeSet<String>,
) -> ValidationReport {
    let mut report = ValidationReport::default();
    let tier_ids = collect_tier_ids(roadmap, &mut report);
    let change_ids = collect_change_ids(roadmap, &mut report);
    validate_change_membership(roadmap, active_changes, &change_ids, &mut report);
    validate_tiers(roadmap, &tier_ids, &mut report);
    validate_changes(
        roadmap,
        &tier_ids,
        &change_ids,
        active_changes,
        accepted_requirements,
        &mut report,
    );
    validate_overlaps(roadmap, &change_ids, active_changes, &mut report);
    validate_change_cycles(roadmap, &change_ids, &mut report);
    validate_tier_cycles(roadmap, &tier_ids, &mut report);
    validate_product_surface_safety(roadmap, &change_ids, &accepted_requirements, &mut report);
    report
}

fn collect_tier_ids(roadmap: &Roadmap, report: &mut ValidationReport) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for tier in &roadmap.tiers {
        if !ids.insert(tier.id.clone()) {
            report.issues.push(issue(
                ISSUE_DUPLICATE_TIER_ID,
                &tier.id,
                "tier appears more than once",
            ));
        }
    }
    ids
}

fn collect_change_ids(roadmap: &Roadmap, report: &mut ValidationReport) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for change in &roadmap.changes {
        if change.id.starts_with(ISSUE_MALFORMED_RECORD) {
            report.issues.push(issue(
                ISSUE_MALFORMED_RECORD,
                &change.id,
                "record has the wrong field count",
            ));
            continue;
        }
        if change.id.starts_with(ISSUE_UNKNOWN_RECORD_KIND) {
            report.issues.push(issue(
                ISSUE_UNKNOWN_RECORD_KIND,
                &change.id,
                "record kind is not supported",
            ));
            continue;
        }
        if !ids.insert(change.id.clone()) {
            report.issues.push(issue(
                ISSUE_DUPLICATE_CHANGE_ID,
                &change.id,
                "change appears more than once",
            ));
        }
    }
    ids
}

fn validate_change_membership(
    roadmap: &Roadmap,
    active_changes: &BTreeSet<String>,
    change_ids: &BTreeSet<String>,
    report: &mut ValidationReport,
) {
    for change in &roadmap.changes {
        if change.id.starts_with("ROADMAP_") {
            continue;
        }
        if !active_changes.contains(&change.id) {
            report.issues.push(issue(
                ISSUE_UNKNOWN_CHANGE_ID,
                &change.id,
                "roadmap classifies a change that is not active",
            ));
        }
    }
    for active_change in active_changes {
        if !change_ids.contains(active_change) {
            report.issues.push(issue(
                ISSUE_UNCLASSIFIED_ACTIVE_CHANGE,
                active_change,
                "active change is missing from the roadmap",
            ));
        }
    }
}

fn validate_tiers(roadmap: &Roadmap, tier_ids: &BTreeSet<String>, report: &mut ValidationReport) {
    for tier in &roadmap.tiers {
        for required_tier in &tier.requires {
            if !tier_ids.contains(required_tier) {
                report.issues.push(issue(
                    ISSUE_UNKNOWN_TIER_ID,
                    &tier.id,
                    &format!("unknown required tier: {required_tier}"),
                ));
            }
        }
    }
}

fn validate_changes(
    roadmap: &Roadmap,
    tier_ids: &BTreeSet<String>,
    change_ids: &BTreeSet<String>,
    active_changes: &BTreeSet<String>,
    accepted_requirements: &BTreeSet<String>,
    report: &mut ValidationReport,
) {
    for change in &roadmap.changes {
        if change.id.starts_with("ROADMAP_") {
            continue;
        }
        if !ALLOWED_ROLES.contains(&change.role.as_str()) {
            report.issues.push(issue(
                ISSUE_UNKNOWN_ROLE,
                &change.id,
                &format!("unknown role: {}", change.role),
            ));
        }
        if !tier_ids.contains(&change.tier) {
            report.issues.push(issue(
                ISSUE_UNKNOWN_TIER_ID,
                &change.id,
                &format!("unknown tier: {}", change.tier),
            ));
        }
        for prerequisite in &change.prerequisites {
            let known_change = change_ids.contains(prerequisite);
            let known_requirement = accepted_requirements.contains(prerequisite);
            if !known_change && !known_requirement {
                report.issues.push(issue(
                    ISSUE_UNKNOWN_PREREQUISITE_ID,
                    &change.id,
                    &format!("unknown prerequisite: {prerequisite}"),
                ));
            }
            if active_changes.contains(prerequisite) {
                report.blocked.push(BlockedChange {
                    change_id: change.id.clone(),
                    prerequisite_id: prerequisite.clone(),
                });
            }
        }
    }
}

fn validate_overlaps(
    roadmap: &Roadmap,
    change_ids: &BTreeSet<String>,
    active_changes: &BTreeSet<String>,
    report: &mut ValidationReport,
) {
    for overlap in &roadmap.overlaps {
        if !change_ids.contains(&overlap.owner) || !active_changes.contains(&overlap.owner) {
            report.issues.push(issue(
                ISSUE_UNKNOWN_OVERLAP_OWNER,
                &overlap.concept,
                &format!("unknown owner: {}", overlap.owner),
            ));
        }
        for related in &overlap.related {
            if !change_ids.contains(related) || !active_changes.contains(related) {
                report.issues.push(issue(
                    ISSUE_UNKNOWN_OVERLAP_RELATED,
                    &overlap.concept,
                    &format!("unknown related change: {related}"),
                ));
            }
        }
    }
}

fn validate_change_cycles(
    roadmap: &Roadmap,
    change_ids: &BTreeSet<String>,
    report: &mut ValidationReport,
) {
    let mut edges = BTreeMap::new();
    for change in &roadmap.changes {
        let known_prerequisites = change
            .prerequisites
            .iter()
            .filter(|prerequisite| change_ids.contains(*prerequisite))
            .cloned()
            .collect();
        edges.insert(change.id.clone(), known_prerequisites);
    }
    for cycle_node in cycle_nodes(change_ids, &edges) {
        report.issues.push(issue(
            ISSUE_DEPENDENCY_CYCLE,
            &cycle_node,
            "change prerequisite cycle detected",
        ));
    }
}

fn validate_tier_cycles(
    roadmap: &Roadmap,
    tier_ids: &BTreeSet<String>,
    report: &mut ValidationReport,
) {
    let mut edges = BTreeMap::new();
    for tier in &roadmap.tiers {
        let known_prerequisites = tier
            .requires
            .iter()
            .filter(|required| tier_ids.contains(*required))
            .cloned()
            .collect();
        edges.insert(tier.id.clone(), known_prerequisites);
    }
    for cycle_node in cycle_nodes(tier_ids, &edges) {
        report.issues.push(issue(
            ISSUE_DEPENDENCY_CYCLE,
            &cycle_node,
            "tier prerequisite cycle detected",
        ));
    }
}

fn cycle_nodes(
    nodes: &BTreeSet<String>,
    prerequisite_edges: &BTreeMap<String, Vec<String>>,
) -> Vec<String> {
    let mut dependent_edges: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut indegree: BTreeMap<String, usize> = BTreeMap::new();
    for node in nodes {
        dependent_edges.entry(node.clone()).or_default();
        indegree.entry(node.clone()).or_default();
    }
    for (node, prerequisites) in prerequisite_edges {
        for prerequisite in prerequisites {
            if nodes.contains(prerequisite) && nodes.contains(node) {
                dependent_edges
                    .entry(prerequisite.clone())
                    .or_default()
                    .push(node.clone());
                *indegree.entry(node.clone()).or_default() += 1;
            }
        }
    }

    let mut ready = VecDeque::new();
    for (node, count) in &indegree {
        if *count == 0 {
            ready.push_back(node.clone());
        }
    }

    let mut processed = BTreeSet::new();
    while let Some(node) = ready.pop_front() {
        if !processed.insert(node.clone()) {
            continue;
        }
        if let Some(dependents) = dependent_edges.get(&node) {
            for dependent in dependents {
                if let Some(count) = indegree.get_mut(dependent) {
                    if *count > 0 {
                        *count -= 1;
                    }
                    if *count == 0 {
                        ready.push_back(dependent.clone());
                    }
                }
            }
        }
    }

    nodes
        .iter()
        .filter(|node| !processed.contains(*node))
        .cloned()
        .collect()
}

fn validate_product_surface_safety(
    roadmap: &Roadmap,
    change_ids: &BTreeSet<String>,
    accepted_requirements: &BTreeSet<String>,
    report: &mut ValidationReport,
) {
    let prerequisite_map = prerequisite_map(roadmap);
    for change in &roadmap.changes {
        if change.role != ROLE_PRODUCT_SURFACE {
            continue;
        }
        let transitive = transitive_prerequisites(
            &change.id,
            &prerequisite_map,
            change_ids,
            accepted_requirements,
        );
        let has_foundation = FOUNDATIONAL_SAFETY_MARKERS
            .iter()
            .any(|id| transitive.contains(*id));
        let has_runtime_safety = RUNTIME_SAFETY_MARKERS
            .iter()
            .any(|id| transitive.contains(*id));
        if !has_foundation || !has_runtime_safety {
            report.issues.push(issue(
                ISSUE_UNSAFE_PRODUCT_SURFACE,
                &change.id,
                "product surface lacks transitive foundational or runtime safety prerequisites",
            ));
        }
    }
}

fn prerequisite_map(roadmap: &Roadmap) -> BTreeMap<String, Vec<String>> {
    let mut map = BTreeMap::new();
    for change in &roadmap.changes {
        map.insert(change.id.clone(), change.prerequisites.clone());
    }
    map
}

fn transitive_prerequisites(
    start: &str,
    prerequisite_map: &BTreeMap<String, Vec<String>>,
    change_ids: &BTreeSet<String>,
    accepted_requirements: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut visited = BTreeSet::new();
    let mut pending = VecDeque::new();
    if let Some(prerequisites) = prerequisite_map.get(start) {
        for prerequisite in prerequisites {
            pending.push_back(prerequisite.clone());
        }
    }
    while let Some(candidate) = pending.pop_front() {
        let known_change = change_ids.contains(&candidate);
        let known_requirement = accepted_requirements.contains(&candidate);
        if !known_change && !known_requirement {
            continue;
        }
        if !visited.insert(candidate.clone()) {
            continue;
        }
        if known_change {
            if let Some(prerequisites) = prerequisite_map.get(&candidate) {
                for prerequisite in prerequisites {
                    pending.push_back(prerequisite.clone());
                }
            }
        }
    }
    visited
}

fn issue(code: &'static str, subject: &str, detail: &str) -> Issue {
    Issue {
        code,
        subject: subject.to_string(),
        detail: detail.to_string(),
    }
}

fn print_report(roadmap: &Roadmap, active_changes: &BTreeSet<String>, report: &ValidationReport) {
    println!("roadmap changes: {}", roadmap.changes.len());
    println!("active changes: {}", active_changes.len());
    println!("blocked dependents: {}", report.blocked.len());
    for blocked in &report.blocked {
        println!(
            "BLOCKED {} waits_for {}",
            blocked.change_id, blocked.prerequisite_id
        );
    }
    if report.issues.is_empty() {
        println!("roadmap validation: ok");
        return;
    }
    for issue in &report.issues {
        println!("ERROR {} {} -- {}", issue.code, issue.subject, issue.detail);
    }
}
