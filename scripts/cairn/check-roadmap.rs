use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::env;
use std::ffi::OsStr;
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const ARG_ROOT: &str = "--root";
const ARG_ROADMAP: &str = "--roadmap";
const ARG_REVIEW_MARKDOWN: &str = "--review-markdown";
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
const MAX_SAFE_FIELD_BYTES: usize = 160;
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
const ISSUE_UNCLASSIFIED_ACTIVE_CHANGE: &str = "ROADMAP_UNCLASSIFIED_ACTIVE_CHANGE";
const ISSUE_UNKNOWN_PREREQUISITE_ID: &str = "ROADMAP_UNKNOWN_PREREQUISITE_ID";
const ISSUE_DEPENDENCY_CYCLE: &str = "ROADMAP_DEPENDENCY_CYCLE";
const ISSUE_UNSAFE_PRODUCT_SURFACE: &str = "ROADMAP_UNSAFE_PRODUCT_SURFACE";
const ISSUE_UNKNOWN_OVERLAP_OWNER: &str = "ROADMAP_UNKNOWN_OVERLAP_OWNER";
const ISSUE_UNKNOWN_OVERLAP_RELATED: &str = "ROADMAP_UNKNOWN_OVERLAP_RELATED";
const ISSUE_PROJECTION_LEAK: &str = "ROADMAP_PROJECTION_LEAK";
const GRAPH_KIND_CHANGE: &str = "change";
const GRAPH_KIND_TIER: &str = "tier";
const BLOCKED_STATUS_ACTIVE_PREREQUISITE: &str = "active-prerequisite";
const SAFETY_CLASS_FOUNDATIONAL: &str = "foundational";
const SAFETY_CLASS_RUNTIME: &str = "runtime";
const SAFETY_MARKER_MISSING_FOUNDATIONAL: &str = "missing-foundational-marker";
const SAFETY_MARKER_MISSING_RUNTIME: &str = "missing-runtime-marker";
const REVIEW_PROJECTION_LINE: &str = "ROADMAP_REVIEW projection=markdown fields=change-id,role,tier,prerequisites,issue-codes,counts redaction=applied";
const REDACTED_FIELD: &str = "[redacted-roadmap-field]";
const EMPTY_FIELD: &str = "-";
const LEAK_PATTERNS: &[&str] = &[
    "/home/",
    "/tmp/",
    "sk-",
    "akia",
    "begin openssh",
    "begin rsa",
    "begin private key",
    "private key",
    "provider_payload",
    "raw prompt",
    "diff --git",
    "command output",
    "token=",
    "secret=",
    "password=",
];

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
    review_markdown: bool,
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
    parse_issues: Vec<Issue>,
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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct TopologicalBatch {
    index: usize,
    changes: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct CyclePath {
    graph_kind: &'static str,
    node_id: String,
    waits_for: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct SafetyMarkerEvidence {
    class: &'static str,
    marker: Option<String>,
    missing_marker_class: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ProductSafetyPath {
    change_id: String,
    active: bool,
    foundational: SafetyMarkerEvidence,
    runtime: SafetyMarkerEvidence,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct ValidationReport {
    issues: Vec<Issue>,
    blocked: Vec<BlockedChange>,
    topological_batches: Vec<TopologicalBatch>,
    cycle_paths: Vec<CyclePath>,
    safety_paths: Vec<ProductSafetyPath>,
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
    sort_report(&mut report);
    print_report(&roadmap, &active_changes, &report);
    if config.review_markdown {
        println!(
            "{}",
            render_markdown_projection(&roadmap, &active_changes, &report)
        );
    }
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
    let mut review_markdown = false;

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
            ARG_REVIEW_MARKDOWN => {
                review_markdown = true;
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    let root = root.unwrap_or_else(|| PathBuf::from("."));
    let roadmap = roadmap.ok_or_else(|| format!("{ARG_ROADMAP} is required"))?;
    Ok(Config {
        root,
        roadmap,
        review_markdown,
    })
}

fn parse_roadmap(text: &str) -> Roadmap {
    let mut roadmap = Roadmap::default();
    for (line_offset, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split(FIELD_SEPARATOR).collect();
        if fields.is_empty() {
            continue;
        }
        let line_number = line_offset + 1;
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
            | RECORD_KIND_ACCEPTED => roadmap.parse_issues.push(issue(
                ISSUE_MALFORMED_RECORD,
                &record_subject(line_number),
                "record has the wrong field count",
            )),
            _unknown => roadmap.parse_issues.push(issue(
                ISSUE_UNKNOWN_RECORD_KIND,
                &record_subject(line_number),
                "record kind is not supported",
            )),
        }
    }
    roadmap
}

fn record_subject(line_number: usize) -> String {
    format!("record:{line_number}")
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
    let mut report = ValidationReport {
        issues: roadmap.parse_issues.clone(),
        ..ValidationReport::default()
    };
    validate_projection_safety(roadmap, active_changes, accepted_requirements, &mut report);
    let tier_ids = collect_tier_ids(roadmap, &mut report);
    let change_ids = collect_change_ids(roadmap, &mut report);
    validate_change_membership(active_changes, &change_ids, &mut report);
    validate_tiers(roadmap, &tier_ids, &mut report);
    validate_changes(
        roadmap,
        &tier_ids,
        &change_ids,
        active_changes,
        accepted_requirements,
        &mut report,
    );
    validate_overlaps(roadmap, &change_ids, &mut report);
    validate_change_cycles(roadmap, &change_ids, &mut report);
    validate_tier_cycles(roadmap, &tier_ids, &mut report);
    validate_product_surface_safety(roadmap, &change_ids, accepted_requirements, &mut report);
    report.topological_batches = topological_batches(
        active_changes,
        &change_prerequisite_edges(roadmap, &change_ids),
    );
    report.safety_paths =
        product_surface_safety_paths(roadmap, &change_ids, accepted_requirements, active_changes);
    report
}

fn validate_projection_safety(
    roadmap: &Roadmap,
    active_changes: &BTreeSet<String>,
    accepted_requirements: &BTreeSet<String>,
    report: &mut ValidationReport,
) {
    for tier in &roadmap.tiers {
        validate_projection_label("tier-id", &tier.id, report);
        for required_tier in &tier.requires {
            validate_projection_label("tier-prerequisite", required_tier, report);
        }
    }
    for change in &roadmap.changes {
        validate_projection_label("change-id", &change.id, report);
        validate_projection_label("role", &change.role, report);
        validate_projection_label("tier-id", &change.tier, report);
        for prerequisite in &change.prerequisites {
            validate_projection_label("change-prerequisite", prerequisite, report);
        }
    }
    for overlap in &roadmap.overlaps {
        validate_projection_label("overlap-owner", &overlap.owner, report);
        for related in &overlap.related {
            validate_projection_label("overlap-related", related, report);
        }
    }
    for active_change in active_changes {
        validate_projection_label("active-change", active_change, report);
    }
    for accepted_requirement in accepted_requirements {
        validate_projection_label("accepted-requirement", accepted_requirement, report);
    }
}

fn validate_projection_label(kind: &'static str, value: &str, report: &mut ValidationReport) {
    if contains_unsafe_material(value) {
        report.issues.push(issue(
            ISSUE_PROJECTION_LEAK,
            kind,
            "roadmap projection label contains unsafe material",
        ));
    }
}

fn contains_unsafe_material(value: &str) -> bool {
    if value.len() > MAX_SAFE_FIELD_BYTES || value.starts_with('/') {
        return true;
    }
    if value.chars().any(char::is_control) {
        return true;
    }
    let lowercase = value.to_ascii_lowercase();
    LEAK_PATTERNS
        .iter()
        .any(|pattern| lowercase.contains(pattern))
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
    active_changes: &BTreeSet<String>,
    change_ids: &BTreeSet<String>,
    report: &mut ValidationReport,
) {
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
    report: &mut ValidationReport,
) {
    for overlap in &roadmap.overlaps {
        if !change_ids.contains(&overlap.owner) {
            report.issues.push(issue(
                ISSUE_UNKNOWN_OVERLAP_OWNER,
                &overlap.concept,
                &format!("unknown owner: {}", overlap.owner),
            ));
        }
        for related in &overlap.related {
            if !change_ids.contains(related) {
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
    let edges = change_prerequisite_edges(roadmap, change_ids);
    let cycle_paths = cycle_paths(change_ids, &edges, GRAPH_KIND_CHANGE);
    for cycle_path in &cycle_paths {
        report.issues.push(issue(
            ISSUE_DEPENDENCY_CYCLE,
            &cycle_path.node_id,
            "change prerequisite cycle detected",
        ));
    }
    report.cycle_paths.extend(cycle_paths);
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
    let cycle_paths = cycle_paths(tier_ids, &edges, GRAPH_KIND_TIER);
    for cycle_path in &cycle_paths {
        report.issues.push(issue(
            ISSUE_DEPENDENCY_CYCLE,
            &cycle_path.node_id,
            "tier prerequisite cycle detected",
        ));
    }
    report.cycle_paths.extend(cycle_paths);
}

fn change_prerequisite_edges(
    roadmap: &Roadmap,
    change_ids: &BTreeSet<String>,
) -> BTreeMap<String, Vec<String>> {
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
    edges
}

fn cycle_paths(
    nodes: &BTreeSet<String>,
    prerequisite_edges: &BTreeMap<String, Vec<String>>,
    graph_kind: &'static str,
) -> Vec<CyclePath> {
    let cycle_nodes: BTreeSet<String> =
        cycle_nodes(nodes, prerequisite_edges).into_iter().collect();
    let mut paths = Vec::new();
    for node in &cycle_nodes {
        let waits_for = prerequisite_edges
            .get(node)
            .and_then(|prerequisites| {
                prerequisites
                    .iter()
                    .find(|prerequisite| cycle_nodes.contains(*prerequisite))
            })
            .cloned()
            .unwrap_or_else(|| node.clone());
        paths.push(CyclePath {
            graph_kind,
            node_id: node.clone(),
            waits_for,
        });
    }
    paths
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

fn topological_batches(
    nodes: &BTreeSet<String>,
    prerequisite_edges: &BTreeMap<String, Vec<String>>,
) -> Vec<TopologicalBatch> {
    let mut dependent_edges: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut indegree: BTreeMap<String, usize> = BTreeMap::new();
    for node in nodes {
        dependent_edges.entry(node.clone()).or_default();
        indegree.entry(node.clone()).or_default();
    }
    for (node, prerequisites) in prerequisite_edges {
        if !nodes.contains(node) {
            continue;
        }
        for prerequisite in prerequisites {
            if nodes.contains(prerequisite) {
                dependent_edges
                    .entry(prerequisite.clone())
                    .or_default()
                    .push(node.clone());
                *indegree.entry(node.clone()).or_default() += 1;
            }
        }
    }

    let mut ready: BTreeSet<String> = indegree
        .iter()
        .filter(|(_node, count)| **count == 0)
        .map(|(node, _count)| node.clone())
        .collect();
    let mut processed = BTreeSet::new();
    let mut batches = Vec::new();

    while !ready.is_empty() {
        let batch_changes: Vec<String> = ready.iter().cloned().collect();
        batches.push(TopologicalBatch {
            index: batches.len(),
            changes: batch_changes.clone(),
        });
        let mut next_ready = BTreeSet::new();
        for node in batch_changes {
            if !processed.insert(node.clone()) {
                continue;
            }
            if let Some(dependents) = dependent_edges.get(&node) {
                for dependent in dependents {
                    if let Some(count) = indegree.get_mut(dependent) {
                        if *count > 0 {
                            *count -= 1;
                        }
                        if *count == 0 && !processed.contains(dependent) {
                            next_ready.insert(dependent.clone());
                        }
                    }
                }
            }
        }
        ready = next_ready;
    }

    batches
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

fn product_surface_safety_paths(
    roadmap: &Roadmap,
    change_ids: &BTreeSet<String>,
    accepted_requirements: &BTreeSet<String>,
    active_changes: &BTreeSet<String>,
) -> Vec<ProductSafetyPath> {
    let prerequisite_map = prerequisite_map(roadmap);
    let mut paths = Vec::new();
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
        paths.push(ProductSafetyPath {
            change_id: change.id.clone(),
            active: active_changes.contains(&change.id),
            foundational: marker_evidence(
                &transitive,
                FOUNDATIONAL_SAFETY_MARKERS,
                SAFETY_CLASS_FOUNDATIONAL,
                SAFETY_MARKER_MISSING_FOUNDATIONAL,
            ),
            runtime: marker_evidence(
                &transitive,
                RUNTIME_SAFETY_MARKERS,
                SAFETY_CLASS_RUNTIME,
                SAFETY_MARKER_MISSING_RUNTIME,
            ),
        });
    }
    paths
}

fn marker_evidence(
    transitive: &BTreeSet<String>,
    markers: &[&str],
    class: &'static str,
    missing_marker_class: &'static str,
) -> SafetyMarkerEvidence {
    let marker = markers
        .iter()
        .find(|marker| transitive.contains(**marker))
        .map(|marker| (*marker).to_string());
    SafetyMarkerEvidence {
        class,
        marker,
        missing_marker_class,
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

fn sort_report(report: &mut ValidationReport) {
    report.issues.sort();
    report.blocked.sort();
    report.topological_batches.sort();
    report.cycle_paths.sort();
    report.safety_paths.sort();
}

fn print_report(roadmap: &Roadmap, active_changes: &BTreeSet<String>, report: &ValidationReport) {
    println!("roadmap changes: {}", roadmap.changes.len());
    println!("active changes: {}", active_changes.len());
    println!("roadmap DAG batches: {}", report.topological_batches.len());
    for batch in &report.topological_batches {
        println!(
            "DAG_BATCH {} {}",
            batch.index,
            safe_join(&batch.changes, ",")
        );
    }
    println!("blocked dependents: {}", report.blocked.len());
    for blocked in &report.blocked {
        println!(
            "BLOCKED {} waits_for {} status {}",
            safe_output_field(&blocked.change_id),
            safe_output_field(&blocked.prerequisite_id),
            BLOCKED_STATUS_ACTIVE_PREREQUISITE,
        );
    }
    println!(
        "product safety paths: total={} missing={}",
        report.safety_paths.len(),
        report
            .safety_paths
            .iter()
            .filter(|path| path.has_missing_marker())
            .count()
    );
    for path in &report.safety_paths {
        if path.active || path.has_missing_marker() {
            println!(
                "SAFETY_PATH {} {} {} {} {}",
                safe_output_field(&path.change_id),
                path.foundational.class,
                safety_marker_label(&path.foundational),
                path.runtime.class,
                safety_marker_label(&path.runtime),
            );
        }
    }
    for cycle_path in &report.cycle_paths {
        println!(
            "CYCLE_PATH {} {} waits_for {}",
            cycle_path.graph_kind,
            safe_output_field(&cycle_path.node_id),
            safe_output_field(&cycle_path.waits_for),
        );
    }
    println!("{REVIEW_PROJECTION_LINE}");
    if report.issues.is_empty() {
        println!("roadmap validation: ok");
        return;
    }
    for issue in &report.issues {
        println!(
            "ERROR {} {} -- {}",
            issue.code,
            safe_output_field(&issue.subject),
            safe_output_field(&issue.detail),
        );
    }
}

impl ProductSafetyPath {
    fn has_missing_marker(&self) -> bool {
        self.foundational.marker.is_none() || self.runtime.marker.is_none()
    }
}

fn safety_marker_label(evidence: &SafetyMarkerEvidence) -> String {
    evidence
        .marker
        .as_ref()
        .map(|marker| safe_output_field(marker))
        .unwrap_or_else(|| evidence.missing_marker_class.to_string())
}

fn safe_join(values: &[String], separator: &str) -> String {
    if values.is_empty() {
        return EMPTY_FIELD.to_string();
    }
    values
        .iter()
        .map(|value| safe_output_field(value))
        .collect::<Vec<_>>()
        .join(separator)
}

fn safe_output_field(value: &str) -> String {
    if value.is_empty() {
        return EMPTY_FIELD.to_string();
    }
    if contains_unsafe_material(value) {
        return REDACTED_FIELD.to_string();
    }
    value.to_string()
}

fn render_markdown_projection(
    _roadmap: &Roadmap,
    active_changes: &BTreeSet<String>,
    report: &ValidationReport,
) -> String {
    let mut output = String::new();
    output.push_str("# Roadmap DAG review\n\n");
    output.push_str("Safe fields: change IDs, roles, tiers, prerequisite IDs, accepted requirement IDs, issue codes, bounded counts, and redaction markers only.\n\n");
    output.push_str("| Count | Value |\n| --- | --- |\n");
    let _ = writeln!(output, "| active changes | {} |", active_changes.len());
    let _ = writeln!(
        output,
        "| topological batches | {} |",
        report.topological_batches.len()
    );
    let _ = writeln!(output, "| blocked dependents | {} |", report.blocked.len());
    let _ = writeln!(output, "| issue count | {} |", report.issues.len());
    output.push_str("\n## Active batches\n\n| Batch | Changes |\n| --- | --- |\n");
    for batch in &report.topological_batches {
        let _ = writeln!(
            output,
            "| {} | {} |",
            batch.index,
            safe_join(&batch.changes, ", ")
        );
    }
    output.push_str(
        "\n## Blocked dependents\n\n| Change | Waits for | Status |\n| --- | --- | --- |\n",
    );
    for blocked in &report.blocked {
        let _ = writeln!(
            output,
            "| {} | {} | {} |",
            safe_output_field(&blocked.change_id),
            safe_output_field(&blocked.prerequisite_id),
            BLOCKED_STATUS_ACTIVE_PREREQUISITE,
        );
    }
    output.push_str("\n## Product safety paths\n\n| Change | Foundational marker | Runtime marker |\n| --- | --- | --- |\n");
    for path in &report.safety_paths {
        if path.active || path.has_missing_marker() {
            let _ = writeln!(
                output,
                "| {} | {} | {} |",
                safe_output_field(&path.change_id),
                safety_marker_label(&path.foundational),
                safety_marker_label(&path.runtime),
            );
        }
    }
    output.push_str("\n## Issues\n\n| Code | Subject |\n| --- | --- |\n");
    for issue in &report.issues {
        let _ = writeln!(
            output,
            "| {} | {} |",
            issue.code,
            safe_output_field(&issue.subject),
        );
    }
    output.push_str(
        "\nRedaction: unsafe projection fields are replaced with `[redacted-roadmap-field]`.\n",
    );
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_DAG: &str = "\
tier|base||Base\n\
tier|runtime|base|Runtime\n\
tier|ui|runtime|UI\n\
active|foundation-change\n\
active|runtime-change\n\
active|ui-surface\n\
accepted|agent_harness_runtime.cairn_policy.closeout\n\
accepted|agent_harness_runtime.validation\n\
change|foundation-change|foundational|base|agent_harness_runtime.cairn_policy.closeout|\n\
change|runtime-change|core-runtime|runtime|foundation-change,agent_harness_runtime.validation|\n\
change|ui-surface|product-surface|ui|runtime-change|";

    const LEAK_DAG: &str = "\
tier|runtime||Runtime\n\
active|/home/example/private\n\
change|/home/example/private|core-runtime|runtime||";

    #[test]
    fn reports_topological_batches_and_safety_paths_for_valid_fixture() {
        let roadmap = parse_roadmap(VALID_DAG);
        let active_changes: BTreeSet<String> = roadmap.active_override.iter().cloned().collect();
        let accepted_requirements: BTreeSet<String> =
            roadmap.accepted_override.iter().cloned().collect();
        let report = validate_roadmap(&roadmap, &active_changes, &accepted_requirements);
        let expected_batches = vec![
            vec!["foundation-change".to_string()],
            vec!["runtime-change".to_string()],
            vec!["ui-surface".to_string()],
        ];
        let actual_batches: Vec<Vec<String>> = report
            .topological_batches
            .iter()
            .map(|batch| batch.changes.clone())
            .collect();
        assert_eq!(report.issues, Vec::new());
        assert_eq!(actual_batches, expected_batches);
        assert!(report.safety_paths.iter().any(|path| {
            path.change_id == "ui-surface"
                && path.foundational.marker.as_deref()
                    == Some("agent_harness_runtime.cairn_policy.closeout")
                && path.runtime.marker.as_deref() == Some("agent_harness_runtime.validation")
        }));
    }

    #[test]
    fn rejects_and_redacts_projection_leak_fixture() {
        let roadmap = parse_roadmap(LEAK_DAG);
        let active_changes: BTreeSet<String> = roadmap.active_override.iter().cloned().collect();
        let accepted_requirements = BTreeSet::new();
        let report = validate_roadmap(&roadmap, &active_changes, &accepted_requirements);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == ISSUE_PROJECTION_LEAK));
        assert_eq!(safe_output_field("/home/example/private"), REDACTED_FIELD);
    }
}
