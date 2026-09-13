use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::PathBuf;

const ARG_ROOT: &str = "--root";
const ARG_CATALOG: &str = "--catalog";
const ARG_DOCS: &str = "--docs";
const COMPONENT_KIND: &str = "component";
const PORT_KIND: &str = "port";
const EVENT_KIND: &str = "event";
const FIELD_SEPARATOR: char = '|';
const LIST_SEPARATOR: char = ',';
const DOC_LIST_SEPARATOR: &str = ", ";
const DOC_ROW_CHECK_SEPARATOR: &str = "<br>";
const COMPONENT_FIELD_COUNT: usize = 12;
const PORT_FIELD_COUNT: usize = 13;
const EVENT_FIELD_COUNT: usize = 11;
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

const CLASS_REUSABLE: &str = "reusable-contract";
const CLASS_HOST_INJECTION: &str = "host-injection-surface";
const CLASS_SHELL: &str = "shell-adapter";
const CLASS_OPTIONAL_BRIDGE: &str = "optional-bridge";
const CLASS_RUNTIME_PRELUDE: &str = "runtime-prelude";
const CLASS_CRATE_FEATURE: &str = "crate-feature";
const AUTHORITY_NONE: &str = "none";
const AUTHORITY_INJECTED: &str = "injected-only";
const AUTHORITY_SHELL: &str = "shell-owned";
const ACTIVATION_REQUIRED: &str = "required";
const ACTIVATION_OPTIONAL: &str = "optional";
const MULTIPLICITY_EXACTLY_ONE: &str = "exactly-one";
const MULTIPLICITY_ZERO_OR_ONE: &str = "zero-or-one";
const MULTIPLICITY_ONE_OR_MORE: &str = "one-or-more";
const CYCLE_ACYCLIC: &str = "acyclic";
const CYCLE_BOUNDED: &str = "bounded";

const ISSUE_MALFORMED_RECORD: &str = "COMPONENT_CATALOG_MALFORMED_RECORD";
const ISSUE_DUPLICATE_ID: &str = "COMPONENT_CATALOG_DUPLICATE_ID";
const ISSUE_INVALID_ID: &str = "COMPONENT_CATALOG_INVALID_ID";
const ISSUE_UNKNOWN_CLASS: &str = "COMPONENT_CATALOG_UNKNOWN_CLASS";
const ISSUE_UNKNOWN_FEATURE: &str = "COMPONENT_CATALOG_UNKNOWN_FEATURE";
const ISSUE_UNKNOWN_AUTHORITY: &str = "COMPONENT_CATALOG_UNKNOWN_AUTHORITY";
const ISSUE_UNKNOWN_FAMILY: &str = "COMPONENT_CATALOG_UNKNOWN_FAMILY";
const ISSUE_UNKNOWN_DEPENDENCY: &str = "COMPONENT_CATALOG_UNKNOWN_DEPENDENCY";
const ISSUE_UNKNOWN_COMPOSITION_LINK: &str = "COMPONENT_CATALOG_UNKNOWN_COMPOSITION_LINK";
const ISSUE_DEPENDENCY_CYCLE: &str = "COMPONENT_CATALOG_DEPENDENCY_CYCLE";
const ISSUE_MISSING_CHECKS: &str = "COMPONENT_CATALOG_MISSING_CHECKS";
const ISSUE_REUSABLE_AMBIENT_AUTHORITY: &str = "COMPONENT_CATALOG_REUSABLE_AMBIENT_AUTHORITY";
const ISSUE_OPTIONAL_BRIDGE_AUTHORITY: &str = "COMPONENT_CATALOG_OPTIONAL_BRIDGE_AUTHORITY";
const ISSUE_UNSAFE_EVIDENCE_FIELD: &str = "COMPONENT_CATALOG_UNSAFE_EVIDENCE_FIELD";
const ISSUE_DOC_DRIFT: &str = "COMPONENT_CATALOG_DOC_DRIFT";
const ISSUE_DUPLICATE_PORT: &str = "COMPONENT_CATALOG_DUPLICATE_PORT";
const ISSUE_UNKNOWN_PORT_OWNER: &str = "COMPONENT_CATALOG_UNKNOWN_PORT_OWNER";
const ISSUE_UNKNOWN_PORT_PROVIDER: &str = "COMPONENT_CATALOG_UNKNOWN_PORT_PROVIDER";
const ISSUE_UNKNOWN_PORT_CONSUMER: &str = "COMPONENT_CATALOG_UNKNOWN_PORT_CONSUMER";
const ISSUE_UNKNOWN_LIFECYCLE_OWNER: &str = "COMPONENT_CATALOG_UNKNOWN_LIFECYCLE_OWNER";
const ISSUE_UNKNOWN_SCOPE: &str = "COMPONENT_CATALOG_UNKNOWN_SCOPE";
const ISSUE_UNKNOWN_MULTIPLICITY: &str = "COMPONENT_CATALOG_UNKNOWN_MULTIPLICITY";
const ISSUE_UNKNOWN_ACTIVATION: &str = "COMPONENT_CATALOG_UNKNOWN_ACTIVATION";
const ISSUE_ORPHAN_REQUIRED_PORT: &str = "COMPONENT_CATALOG_ORPHAN_REQUIRED_PORT";
const ISSUE_INVALID_MULTIPLICITY: &str = "COMPONENT_CATALOG_INVALID_MULTIPLICITY";
const ISSUE_PORT_AUTHORITY_MISMATCH: &str = "COMPONENT_CATALOG_PORT_AUTHORITY_MISMATCH";
const ISSUE_DUPLICATE_EVENT: &str = "COMPONENT_CATALOG_DUPLICATE_EVENT";
const ISSUE_UNKNOWN_EVENT_PRODUCER: &str = "COMPONENT_CATALOG_UNKNOWN_EVENT_PRODUCER";
const ISSUE_UNKNOWN_EVENT_CONSUMER: &str = "COMPONENT_CATALOG_UNKNOWN_EVENT_CONSUMER";
const ISSUE_UNKNOWN_DELIVERY: &str = "COMPONENT_CATALOG_UNKNOWN_DELIVERY";
const ISSUE_UNKNOWN_DURABILITY: &str = "COMPONENT_CATALOG_UNKNOWN_DURABILITY";
const ISSUE_UNKNOWN_CYCLE_POLICY: &str = "COMPONENT_CATALOG_UNKNOWN_CYCLE_POLICY";
const ISSUE_MISSING_EVENT_CONSUMER: &str = "COMPONENT_CATALOG_MISSING_EVENT_CONSUMER";
const ISSUE_UNSUPPORTED_EVENT_CYCLE: &str = "COMPONENT_CATALOG_UNSUPPORTED_EVENT_CYCLE";
const ISSUE_PROFILE_DRIFT: &str = "COMPONENT_CATALOG_PROFILE_DRIFT";

const LIVE_STEEL_CONSUMER: &str = "std.live-steel-agent-shell";
const LIVE_STEEL_PROFILE_PATH: &str = "crates/animus-live-steel-agent/src/readiness/mod.rs";
const LIVE_TERMINAL_CONSUMER: &str = "std.live-terminal-agent-host";
const LIVE_TERMINAL_PROFILE_PATH: &str = "crates/animus-live-terminal/src/activation.rs";

const ALLOWED_CLASSES: &[&str] = &[
    CLASS_REUSABLE,
    CLASS_HOST_INJECTION,
    CLASS_SHELL,
    CLASS_OPTIONAL_BRIDGE,
    CLASS_RUNTIME_PRELUDE,
    CLASS_CRATE_FEATURE,
];

const ALLOWED_FEATURES: &[&str] = &[
    "always",
    "default",
    "compat",
    "router",
    "runtime",
    "status",
    "orchestration",
    "steel",
    "preserves",
    "std",
    "tui",
    "all-families",
    "batch",
    "compaction",
    "dataset-export",
    "headless",
    "loop",
    "mesh-provider",
    "harbor-eval",
    "harbor-atif",
    "basalt-authority-adapter",
    "spacewasm-diagnostic",
    "model-catalog",
    "model-runtime-profiles",
    "openai-provider-host",
    "live-provider",
    "local-daemon-supervisor-host",
    "live-terminal",
    "agent-role-profiles",
    "active-turn-recovery",
    "pending-input-vault",
    "daemon-handoff",
    "workspace-edit-host",
    "mcp-client-host",
    "pueue-host",
    "conversation-view-projection",
    "session-branch-explorer",
    "provider-retry",
    "rejected-attempt-context",
    "branchfs-host",
    "coding-session-host",
    "tool-execution-stages",
    "harness-convergence",
    "child-session-host",
    "codebase-graph-host",
    "codebase-tool-host",
    "acp-stdio-host",
    "agent-control",
    "agent-guardrail",
    "lean-loop",
    "rlm-kernel",
    "steel-skill-modules",
    "rlm-host-executor",
    "sandlock-host",
    "session-tree-export",
    "durable-session-store",
    "durable-steel-kernel-restore",
    "iroh-host",
    "syndicate-host",
    "iroh-syndicate-host",
    "policy",
    "provenance",
    "quality",
    "research",
    "scaffold",
    "worker-team",
    "weave-adapter",
    "autonomous-gates",
    "continual-harness",
    "daemon-workers",
    "messaging-receipts",
    "persistent-goals",
    "schedule-reliability",
    "session-event-replay",
    "execution-backend-planner",
    "pijul-vcs-adapter",
    "trajectory-transcript-adapter",
    "secretspec-host-resolution",
    "native-evaluation",
    "evaluation-adapters",
    "change-request-adapters",
    "maintenance-adapters",
    "evaluation-host",
    "evaluation-conformance",
    "terminal-image",
    "terminal-video",
];

const ALLOWED_AUTHORITIES: &[&str] = &[
    AUTHORITY_NONE,
    AUTHORITY_INJECTED,
    AUTHORITY_SHELL,
    "terminal-owned",
    "process-owned",
    "filesystem-owned",
    "network-owned",
    "persistence-owned",
    "credential-owned",
];

const ALLOWED_FAMILIES: &[&str] = &[
    "adapter-contracts",
    "agent-control",
    "agent-role",
    "compatibility",
    "background-queue",
    "batch",
    "compaction",
    "component-catalog",
    "context",
    "core",
    "dataset-export",
    "document-ingest",
    "credentials",
    "evaluation",
    "headless",
    "model",
    "model-evidence",
    "observability",
    "orchestration",
    "policy",
    "preserves",
    "provenance",
    "quality",
    "research",
    "router",
    "runtime",
    "safe-receipt",
    "security-policy",
    "scaffold",
    "status",
    "std-shell",
    "steel",
    "tool",
    "tui",
    "weave",
    "worker-team",
    "workspace",
    "autonomous-gates",
    "refinement",
    "daemon",
    "messaging",
    "goal-state",
    "schedule",
    "review",
    "maintenance",
];

const ALLOWED_SCOPES: &[&str] = &["application", "session", "worker", "client", "request"];
const ALLOWED_MULTIPLICITIES: &[&str] = &[
    MULTIPLICITY_EXACTLY_ONE,
    MULTIPLICITY_ZERO_OR_ONE,
    MULTIPLICITY_ONE_OR_MORE,
];
const ALLOWED_ACTIVATIONS: &[&str] = &[ACTIVATION_REQUIRED, ACTIVATION_OPTIONAL];
const ALLOWED_DELIVERIES: &[&str] = &[
    "ordered",
    "snapshot-first",
    "serial",
    "parallel",
    "waterfall",
];
const ALLOWED_DURABILITIES: &[&str] = &["durable", "live-only", "metadata-only"];
const ALLOWED_CYCLE_POLICIES: &[&str] = &[CYCLE_ACYCLIC, CYCLE_BOUNDED];

const UNSAFE_FIELD_MARKERS: &[&str] = &[
    "/home/",
    "api_key=",
    "aws_secret_access_key",
    "begin private key",
    "credential=",
    "credential-value",
    "credential_value",
    "private-path",
    "provider-payload",
    "provider_payload",
    "raw-file",
    "raw-prompt",
    "raw-transcript",
    "raw_prompt",
    "secret=",
    "secret-value",
    "secret_value",
    "sk-",
    "unbounded-output",
    "unbounded_output",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    root: PathBuf,
    catalog: PathBuf,
    docs: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ComponentEntry {
    component_id: String,
    owner: String,
    component_class: String,
    feature_gate: String,
    required_authority: String,
    depends_on: Vec<String>,
    composes_with: Vec<String>,
    capability_family: String,
    focused_checks: Vec<String>,
    evidence_labels: Vec<String>,
    public_import: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PortEntry {
    port_id: String,
    owner_component_id: String,
    provider_component_ids: Vec<String>,
    consumer_component_ids: Vec<String>,
    scope_class: String,
    multiplicity: String,
    lifecycle_owner_component_id: String,
    activation: String,
    feature_gate: String,
    required_authority: String,
    focused_check: String,
    evidence_labels: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EventEntry {
    event_id: String,
    producer_component_id: String,
    consumer_component_ids: Vec<String>,
    delivery: String,
    durability: String,
    lifecycle_owner_component_id: String,
    activation: String,
    cycle_policy: String,
    focused_check: String,
    evidence_labels: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Catalog {
    components: Vec<ComponentEntry>,
    ports: Vec<PortEntry>,
    events: Vec<EventEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Issue {
    code: &'static str,
    subject: String,
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
    let catalog_text = fs::read_to_string(&config.catalog)
        .map_err(|error| format!("failed to read component catalog: {error}"))?;
    let (catalog, mut issues) = parse_catalog(&catalog_text);
    issues.extend(validate_catalog(&catalog));
    issues.extend(validate_profile_projection(&config.root, &catalog));

    if let Some(docs_path) = config.docs {
        let docs_text = fs::read_to_string(&docs_path)
            .map_err(|error| format!("failed to read component catalog docs: {error}"))?;
        let expected_docs = render_docs(&catalog);
        if docs_text != expected_docs {
            issues.push(Issue {
                code: ISSUE_DOC_DRIFT,
                subject: docs_path.display().to_string(),
                detail: "docs/component-catalog.md does not match Nickel catalog projection"
                    .to_string(),
            });
        }
    }

    issues.sort();
    print_report(&catalog, &issues);
    if issues.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "component catalog failed with {} issue(s)",
            issues.len()
        ))
    }
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
    let _program = args.next();
    let mut root = PathBuf::from(".");
    let mut catalog = None;
    let mut docs = None;

    while let Some(argument) = args.next() {
        match argument.as_str() {
            ARG_ROOT => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_ROOT} requires a path"))?;
                root = PathBuf::from(value);
            }
            ARG_CATALOG => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_CATALOG} requires a path"))?;
                catalog = Some(PathBuf::from(value));
            }
            ARG_DOCS => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_DOCS} requires a path"))?;
                docs = Some(PathBuf::from(value));
            }
            _ => return Err(format!("unknown argument: {argument}")),
        }
    }

    let catalog = catalog.unwrap_or_else(|| root.join(".cairn/component-catalog.ncl.lines"));
    Ok(Config {
        root,
        catalog,
        docs,
    })
}

fn parse_catalog(text: &str) -> (Catalog, Vec<Issue>) {
    let mut catalog = Catalog::default();
    let mut issues = Vec::new();

    for (line_index, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        let parts = line.split(FIELD_SEPARATOR).collect::<Vec<_>>();
        let kind = parts.first().copied().unwrap_or_default();
        let parse_result = match kind {
            COMPONENT_KIND if parts.len() == COMPONENT_FIELD_COUNT => {
                catalog.components.push(parse_component(&parts));
                Ok(())
            }
            PORT_KIND if parts.len() == PORT_FIELD_COUNT => {
                catalog.ports.push(parse_port(&parts));
                Ok(())
            }
            EVENT_KIND if parts.len() == EVENT_FIELD_COUNT => {
                catalog.events.push(parse_event(&parts));
                Ok(())
            }
            _ => Err(()),
        };
        if parse_result.is_err() {
            issues.push(Issue {
                code: ISSUE_MALFORMED_RECORD,
                subject: format!("line {}", display_line_number(line_index)),
                detail: format!(
                    "expected a {COMPONENT_FIELD_COUNT}-field component, {PORT_FIELD_COUNT}-field port, or {EVENT_FIELD_COUNT}-field event"
                ),
            });
        }
    }

    (catalog, issues)
}

fn parse_component(parts: &[&str]) -> ComponentEntry {
    ComponentEntry {
        component_id: parts[1].to_string(),
        owner: parts[2].to_string(),
        component_class: parts[3].to_string(),
        feature_gate: parts[4].to_string(),
        required_authority: parts[5].to_string(),
        depends_on: parse_list(parts[6]),
        composes_with: parse_list(parts[7]),
        capability_family: parts[8].to_string(),
        focused_checks: parse_list(parts[9]),
        evidence_labels: parse_list(parts[10]),
        public_import: parts[11].to_string(),
    }
}

fn parse_port(parts: &[&str]) -> PortEntry {
    PortEntry {
        port_id: parts[1].to_string(),
        owner_component_id: parts[2].to_string(),
        provider_component_ids: parse_list(parts[3]),
        consumer_component_ids: parse_list(parts[4]),
        scope_class: parts[5].to_string(),
        multiplicity: parts[6].to_string(),
        lifecycle_owner_component_id: parts[7].to_string(),
        activation: parts[8].to_string(),
        feature_gate: parts[9].to_string(),
        required_authority: parts[10].to_string(),
        focused_check: parts[11].to_string(),
        evidence_labels: parse_list(parts[12]),
    }
}

fn parse_event(parts: &[&str]) -> EventEntry {
    EventEntry {
        event_id: parts[1].to_string(),
        producer_component_id: parts[2].to_string(),
        consumer_component_ids: parse_list(parts[3]),
        delivery: parts[4].to_string(),
        durability: parts[5].to_string(),
        lifecycle_owner_component_id: parts[6].to_string(),
        activation: parts[7].to_string(),
        cycle_policy: parts[8].to_string(),
        focused_check: parts[9].to_string(),
        evidence_labels: parse_list(parts[10]),
    }
}

fn display_line_number(line_index: usize) -> usize {
    line_index + 1
}

fn parse_list(field: &str) -> Vec<String> {
    if field.trim().is_empty() {
        Vec::new()
    } else {
        field
            .split(LIST_SEPARATOR)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
            .collect()
    }
}

fn validate_profile_projection(root: &PathBuf, catalog: &Catalog) -> Vec<Issue> {
    let profiles = [
        (LIVE_STEEL_CONSUMER, LIVE_STEEL_PROFILE_PATH),
        (LIVE_TERMINAL_CONSUMER, LIVE_TERMINAL_PROFILE_PATH),
    ];
    let mut issues = Vec::new();
    for (consumer, relative_path) in profiles {
        let ports = catalog
            .ports
            .iter()
            .filter(|port| {
                port.consumer_component_ids
                    .iter()
                    .any(|value| value == consumer)
            })
            .collect::<Vec<_>>();
        let events = catalog
            .events
            .iter()
            .filter(|event| {
                event
                    .consumer_component_ids
                    .iter()
                    .any(|value| value == consumer)
            })
            .collect::<Vec<_>>();
        if ports.is_empty() && events.is_empty() {
            continue;
        }
        let profile_path = root.join(relative_path);
        let profile_source = match fs::read_to_string(&profile_path) {
            Ok(source) => source,
            Err(error) => {
                issues.push(issue(
                    ISSUE_PROFILE_DRIFT,
                    relative_path,
                    &format!("failed to read maintained activation profile: {error}"),
                ));
                continue;
            }
        };
        for port in ports {
            let mut values = vec![
                port.port_id.as_str(),
                port.owner_component_id.as_str(),
                port.lifecycle_owner_component_id.as_str(),
                port.scope_class.as_str(),
                port.feature_gate.as_str(),
            ];
            values.extend(port.provider_component_ids.iter().map(String::as_str));
            values.extend(port.consumer_component_ids.iter().map(String::as_str));
            validate_profile_values(
                &profile_source,
                relative_path,
                port.port_id.as_str(),
                values,
                &mut issues,
            );
        }
        for event in events {
            let mut values = vec![
                event.event_id.as_str(),
                event.producer_component_id.as_str(),
                event.lifecycle_owner_component_id.as_str(),
            ];
            values.extend(event.consumer_component_ids.iter().map(String::as_str));
            validate_profile_values(
                &profile_source,
                relative_path,
                event.event_id.as_str(),
                values,
                &mut issues,
            );
        }
    }
    issues
}

fn validate_profile_values(
    source: &str,
    relative_path: &str,
    subject: &str,
    values: Vec<&str>,
    issues: &mut Vec<Issue>,
) {
    let missing = values
        .into_iter()
        .filter(|value| !source.contains(value))
        .collect::<BTreeSet<_>>();
    if !missing.is_empty() {
        issues.push(issue(
            ISSUE_PROFILE_DRIFT,
            subject,
            &format!(
                "{relative_path} omits catalog values: {}",
                missing
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join(DOC_LIST_SEPARATOR)
            ),
        ));
    }
}

fn validate_catalog(catalog: &Catalog) -> Vec<Issue> {
    let mut issues = Vec::new();
    let mut all_ids = BTreeSet::new();
    for entry in &catalog.components {
        if !all_ids.insert(entry.component_id.clone()) {
            issues.push(issue(
                ISSUE_DUPLICATE_ID,
                entry.component_id.as_str(),
                "component id appears more than once",
            ));
        }
    }
    for entry in &catalog.components {
        validate_component(entry, &all_ids, &mut issues);
    }
    issues.extend(validate_dependency_cycles(&catalog.components));
    validate_ports(catalog, &all_ids, &mut issues);
    validate_events(catalog, &all_ids, &mut issues);
    issues.extend(validate_event_cycles(&catalog.events));
    issues
}

fn validate_component(entry: &ComponentEntry, all_ids: &BTreeSet<String>, issues: &mut Vec<Issue>) {
    if !is_valid_id(&entry.component_id) {
        issues.push(issue(
            ISSUE_INVALID_ID,
            entry.component_id.as_str(),
            "component ids must use lowercase letters, digits, '.', and '-'",
        ));
    }
    if !ALLOWED_CLASSES.contains(&entry.component_class.as_str()) {
        issues.push(issue(
            ISSUE_UNKNOWN_CLASS,
            entry.component_id.as_str(),
            entry.component_class.as_str(),
        ));
    }
    if !ALLOWED_FEATURES.contains(&entry.feature_gate.as_str()) {
        issues.push(issue(
            ISSUE_UNKNOWN_FEATURE,
            entry.component_id.as_str(),
            entry.feature_gate.as_str(),
        ));
    }
    if !ALLOWED_AUTHORITIES.contains(&entry.required_authority.as_str()) {
        issues.push(issue(
            ISSUE_UNKNOWN_AUTHORITY,
            entry.component_id.as_str(),
            entry.required_authority.as_str(),
        ));
    }
    if !ALLOWED_FAMILIES.contains(&entry.capability_family.as_str()) {
        issues.push(issue(
            ISSUE_UNKNOWN_FAMILY,
            entry.component_id.as_str(),
            entry.capability_family.as_str(),
        ));
    }
    if entry.focused_checks.is_empty() {
        issues.push(issue(
            ISSUE_MISSING_CHECKS,
            entry.component_id.as_str(),
            "component entries must name at least one focused check",
        ));
    }
    if entry.component_class == CLASS_REUSABLE && entry.required_authority != AUTHORITY_NONE {
        issues.push(issue(
            ISSUE_REUSABLE_AMBIENT_AUTHORITY,
            entry.component_id.as_str(),
            format!(
                "reusable contract requires {} authority",
                entry.required_authority
            )
            .as_str(),
        ));
    }
    if entry.component_class == CLASS_OPTIONAL_BRIDGE && entry.required_authority != AUTHORITY_NONE
    {
        issues.push(issue(
            ISSUE_OPTIONAL_BRIDGE_AUTHORITY,
            entry.component_id.as_str(),
            format!(
                "optional bridge requires {} authority",
                entry.required_authority
            )
            .as_str(),
        ));
    }
    for dependency in &entry.depends_on {
        if !all_ids.contains(dependency) {
            issues.push(issue(
                ISSUE_UNKNOWN_DEPENDENCY,
                entry.component_id.as_str(),
                dependency.as_str(),
            ));
        }
    }
    for link in &entry.composes_with {
        if !all_ids.contains(link) {
            issues.push(issue(
                ISSUE_UNKNOWN_COMPOSITION_LINK,
                entry.component_id.as_str(),
                link.as_str(),
            ));
        }
    }
    validate_safe_fields(
        entry.component_id.as_str(),
        [
            ("component_id", entry.component_id.as_str()),
            ("owner", entry.owner.as_str()),
            ("feature_gate", entry.feature_gate.as_str()),
            ("capability_family", entry.capability_family.as_str()),
            ("public_import", entry.public_import.as_str()),
        ],
        issues,
    );
    validate_safe_list(
        entry.component_id.as_str(),
        "focused_checks",
        &entry.focused_checks,
        issues,
    );
    validate_safe_list(
        entry.component_id.as_str(),
        "evidence_labels",
        &entry.evidence_labels,
        issues,
    );
}

fn validate_ports(catalog: &Catalog, all_ids: &BTreeSet<String>, issues: &mut Vec<Issue>) {
    let component_map = catalog
        .components
        .iter()
        .map(|entry| (entry.component_id.as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    let mut port_ids = BTreeSet::new();
    for port in &catalog.ports {
        if !is_valid_id(&port.port_id) {
            issues.push(issue(
                ISSUE_INVALID_ID,
                port.port_id.as_str(),
                "port ids must use lowercase letters, digits, '.', and '-'",
            ));
        }
        if !port_ids.insert(port.port_id.as_str()) {
            issues.push(issue(
                ISSUE_DUPLICATE_PORT,
                port.port_id.as_str(),
                "port id appears more than once",
            ));
        }
        validate_known_ref(
            all_ids,
            port.owner_component_id.as_str(),
            ISSUE_UNKNOWN_PORT_OWNER,
            port.port_id.as_str(),
            issues,
        );
        validate_known_ref(
            all_ids,
            port.lifecycle_owner_component_id.as_str(),
            ISSUE_UNKNOWN_LIFECYCLE_OWNER,
            port.port_id.as_str(),
            issues,
        );
        for provider in &port.provider_component_ids {
            validate_known_ref(
                all_ids,
                provider.as_str(),
                ISSUE_UNKNOWN_PORT_PROVIDER,
                port.port_id.as_str(),
                issues,
            );
        }
        for consumer in &port.consumer_component_ids {
            validate_known_ref(
                all_ids,
                consumer.as_str(),
                ISSUE_UNKNOWN_PORT_CONSUMER,
                port.port_id.as_str(),
                issues,
            );
        }
        validate_choice(
            &port.scope_class,
            ALLOWED_SCOPES,
            ISSUE_UNKNOWN_SCOPE,
            port.port_id.as_str(),
            issues,
        );
        validate_choice(
            &port.multiplicity,
            ALLOWED_MULTIPLICITIES,
            ISSUE_UNKNOWN_MULTIPLICITY,
            port.port_id.as_str(),
            issues,
        );
        validate_choice(
            &port.activation,
            ALLOWED_ACTIVATIONS,
            ISSUE_UNKNOWN_ACTIVATION,
            port.port_id.as_str(),
            issues,
        );
        validate_choice(
            &port.feature_gate,
            ALLOWED_FEATURES,
            ISSUE_UNKNOWN_FEATURE,
            port.port_id.as_str(),
            issues,
        );
        validate_choice(
            &port.required_authority,
            ALLOWED_AUTHORITIES,
            ISSUE_UNKNOWN_AUTHORITY,
            port.port_id.as_str(),
            issues,
        );
        if port.focused_check.is_empty() {
            issues.push(issue(
                ISSUE_MISSING_CHECKS,
                port.port_id.as_str(),
                "port entries must name one focused check",
            ));
        }
        if port.activation == ACTIVATION_REQUIRED
            && (port.provider_component_ids.is_empty() || port.consumer_component_ids.is_empty())
        {
            issues.push(issue(
                ISSUE_ORPHAN_REQUIRED_PORT,
                port.port_id.as_str(),
                "required port must declare providers and direct consumers",
            ));
        }
        if !multiplicity_admits(&port.multiplicity, port.provider_component_ids.len()) {
            issues.push(issue(
                ISSUE_INVALID_MULTIPLICITY,
                port.port_id.as_str(),
                port.multiplicity.as_str(),
            ));
        }
        for provider in &port.provider_component_ids {
            if let Some(component) = component_map.get(provider.as_str()) {
                if !authority_supplies(
                    component.required_authority.as_str(),
                    port.required_authority.as_str(),
                ) {
                    issues.push(issue(
                        ISSUE_PORT_AUTHORITY_MISMATCH,
                        port.port_id.as_str(),
                        provider.as_str(),
                    ));
                }
            }
        }
        validate_safe_fields(
            port.port_id.as_str(),
            [
                ("port_id", port.port_id.as_str()),
                ("scope_class", port.scope_class.as_str()),
                ("focused_check", port.focused_check.as_str()),
            ],
            issues,
        );
        validate_safe_list(
            port.port_id.as_str(),
            "evidence_labels",
            &port.evidence_labels,
            issues,
        );
    }
}

fn validate_events(catalog: &Catalog, all_ids: &BTreeSet<String>, issues: &mut Vec<Issue>) {
    let mut event_ids = BTreeSet::new();
    for event in &catalog.events {
        if !is_valid_id(&event.event_id) {
            issues.push(issue(
                ISSUE_INVALID_ID,
                event.event_id.as_str(),
                "event ids must use lowercase letters, digits, '.', and '-'",
            ));
        }
        if !event_ids.insert(event.event_id.as_str()) {
            issues.push(issue(
                ISSUE_DUPLICATE_EVENT,
                event.event_id.as_str(),
                "event id appears more than once",
            ));
        }
        validate_known_ref(
            all_ids,
            event.producer_component_id.as_str(),
            ISSUE_UNKNOWN_EVENT_PRODUCER,
            event.event_id.as_str(),
            issues,
        );
        validate_known_ref(
            all_ids,
            event.lifecycle_owner_component_id.as_str(),
            ISSUE_UNKNOWN_LIFECYCLE_OWNER,
            event.event_id.as_str(),
            issues,
        );
        for consumer in &event.consumer_component_ids {
            validate_known_ref(
                all_ids,
                consumer.as_str(),
                ISSUE_UNKNOWN_EVENT_CONSUMER,
                event.event_id.as_str(),
                issues,
            );
        }
        validate_choice(
            &event.delivery,
            ALLOWED_DELIVERIES,
            ISSUE_UNKNOWN_DELIVERY,
            event.event_id.as_str(),
            issues,
        );
        validate_choice(
            &event.durability,
            ALLOWED_DURABILITIES,
            ISSUE_UNKNOWN_DURABILITY,
            event.event_id.as_str(),
            issues,
        );
        validate_choice(
            &event.activation,
            ALLOWED_ACTIVATIONS,
            ISSUE_UNKNOWN_ACTIVATION,
            event.event_id.as_str(),
            issues,
        );
        validate_choice(
            &event.cycle_policy,
            ALLOWED_CYCLE_POLICIES,
            ISSUE_UNKNOWN_CYCLE_POLICY,
            event.event_id.as_str(),
            issues,
        );
        if event.focused_check.is_empty() {
            issues.push(issue(
                ISSUE_MISSING_CHECKS,
                event.event_id.as_str(),
                "event entries must name one focused check",
            ));
        }
        if event.activation == ACTIVATION_REQUIRED && event.consumer_component_ids.is_empty() {
            issues.push(issue(
                ISSUE_MISSING_EVENT_CONSUMER,
                event.event_id.as_str(),
                "required event must declare at least one consumer",
            ));
        }
        validate_safe_fields(
            event.event_id.as_str(),
            [
                ("event_id", event.event_id.as_str()),
                ("focused_check", event.focused_check.as_str()),
            ],
            issues,
        );
        validate_safe_list(
            event.event_id.as_str(),
            "evidence_labels",
            &event.evidence_labels,
            issues,
        );
    }
}

fn validate_known_ref(
    all_ids: &BTreeSet<String>,
    value: &str,
    code: &'static str,
    subject: &str,
    issues: &mut Vec<Issue>,
) {
    if !all_ids.contains(value) {
        issues.push(issue(code, subject, value));
    }
}

fn validate_choice(
    value: &str,
    allowed: &[&str],
    code: &'static str,
    subject: &str,
    issues: &mut Vec<Issue>,
) {
    if !allowed.contains(&value) {
        issues.push(issue(code, subject, value));
    }
}

fn multiplicity_admits(multiplicity: &str, count: usize) -> bool {
    match multiplicity {
        MULTIPLICITY_EXACTLY_ONE => count == 1,
        MULTIPLICITY_ZERO_OR_ONE => count <= 1,
        MULTIPLICITY_ONE_OR_MORE => count >= 1,
        _ => true,
    }
}

fn authority_supplies(provider: &str, required: &str) -> bool {
    match required {
        AUTHORITY_NONE => true,
        AUTHORITY_INJECTED => provider == AUTHORITY_INJECTED || provider == AUTHORITY_SHELL,
        AUTHORITY_SHELL => provider == AUTHORITY_SHELL,
        _ => provider == required,
    }
}

fn is_valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.chars().all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || character == '.'
                || character == '-'
        })
}

fn validate_safe_fields<const FIELD_COUNT: usize>(
    subject: &str,
    fields: [(&str, &str); FIELD_COUNT],
    issues: &mut Vec<Issue>,
) {
    for (field_name, field_value) in fields {
        if contains_unsafe_marker(field_value) {
            issues.push(issue(ISSUE_UNSAFE_EVIDENCE_FIELD, subject, field_name));
        }
    }
}

fn validate_safe_list(subject: &str, field_name: &str, values: &[String], issues: &mut Vec<Issue>) {
    for value in values {
        if contains_unsafe_marker(value) {
            issues.push(issue(ISSUE_UNSAFE_EVIDENCE_FIELD, subject, field_name));
        }
    }
}

fn contains_unsafe_marker(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();
    UNSAFE_FIELD_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
}

fn validate_dependency_cycles(entries: &[ComponentEntry]) -> Vec<Issue> {
    let graph = entries
        .iter()
        .map(|entry| (entry.component_id.as_str(), entry.depends_on.as_slice()))
        .collect::<BTreeMap<_, _>>();
    let mut issues = Vec::new();
    let mut visited = BTreeSet::new();
    let mut visiting = BTreeSet::new();
    for entry in entries {
        detect_dependency_cycle(
            entry.component_id.as_str(),
            &graph,
            &mut visiting,
            &mut visited,
            &mut issues,
        );
    }
    issues
}

fn detect_dependency_cycle<'a>(
    node: &'a str,
    graph: &BTreeMap<&'a str, &'a [String]>,
    visiting: &mut BTreeSet<&'a str>,
    visited: &mut BTreeSet<&'a str>,
    issues: &mut Vec<Issue>,
) {
    if visited.contains(node) {
        return;
    }
    if !visiting.insert(node) {
        issues.push(issue(
            ISSUE_DEPENDENCY_CYCLE,
            node,
            "dependency cycle detected",
        ));
        return;
    }
    if let Some(dependencies) = graph.get(node) {
        for dependency in *dependencies {
            if graph.contains_key(dependency.as_str()) {
                detect_dependency_cycle(dependency.as_str(), graph, visiting, visited, issues);
            }
        }
    }
    visiting.remove(node);
    visited.insert(node);
}

fn validate_event_cycles(events: &[EventEntry]) -> Vec<Issue> {
    let mut edges = BTreeMap::<&str, Vec<&str>>::new();
    for event in events {
        for consumer in &event.consumer_component_ids {
            edges
                .entry(event.producer_component_id.as_str())
                .or_default()
                .push(consumer.as_str());
        }
    }
    let mut issues = Vec::new();
    for event in events
        .iter()
        .filter(|event| event.cycle_policy == CYCLE_ACYCLIC)
    {
        for consumer in &event.consumer_component_ids {
            let mut visited = BTreeSet::new();
            if path_exists(
                consumer.as_str(),
                event.producer_component_id.as_str(),
                &edges,
                &mut visited,
            ) {
                issues.push(issue(
                    ISSUE_UNSUPPORTED_EVENT_CYCLE,
                    event.event_id.as_str(),
                    "event cycle includes an acyclic edge",
                ));
            }
        }
    }
    issues
}

fn path_exists<'a>(
    current: &'a str,
    target: &str,
    edges: &BTreeMap<&'a str, Vec<&'a str>>,
    visited: &mut BTreeSet<&'a str>,
) -> bool {
    if current == target {
        return true;
    }
    if !visited.insert(current) {
        return false;
    }
    edges.get(current).is_some_and(|next_values| {
        next_values
            .iter()
            .any(|next| path_exists(next, target, edges, visited))
    })
}

fn render_docs(catalog: &Catalog) -> String {
    let mut text = String::new();
    text.push_str("# Component catalog\n\n");
    text.push_str("This projection is generated from `.cairn/component-catalog.ncl`. Do not edit catalog rows by hand; update the Nickel source and run `scripts/cairn/check-component-catalog --root .`.\n\n");
    text.push_str("The catalog names Lego-style Animus components, application-owned capability seams, event ownership, activation posture, and focused rails. It is metadata evidence only. It does not prove runtime readiness, event delivery, effect success, authority, production readiness, or release eligibility.\n\n");
    text.push_str("| Component | Class | Owner | Feature gate | Authority | Family | Focused checks | Smallest import |\n");
    text.push_str("| --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for entry in &catalog.components {
        text.push_str(format_component_row(entry).as_str());
    }
    text.push_str("\n## Dependency map\n\n");
    text.push_str("| Component | Dependencies | Composition links |\n");
    text.push_str("| --- | --- | --- |\n");
    for entry in &catalog.components {
        text.push_str("| `");
        text.push_str(&entry.component_id);
        text.push_str("` | ");
        text.push_str(code_list(&entry.depends_on).as_str());
        text.push_str(" | ");
        text.push_str(code_list(&entry.composes_with).as_str());
        text.push_str(" |\n");
    }
    text.push_str("\n## Capability seams\n\n");
    text.push_str("| Port | Owner | Providers | Consumers | Scope | Multiplicity | Lifecycle owner | Activation | Feature | Authority | Focused check |\n");
    text.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for port in &catalog.ports {
        text.push_str("| `");
        text.push_str(&port.port_id);
        text.push_str("` | `");
        text.push_str(&port.owner_component_id);
        text.push_str("` | ");
        text.push_str(code_list(&port.provider_component_ids).as_str());
        text.push_str(" | ");
        text.push_str(code_list(&port.consumer_component_ids).as_str());
        text.push_str(" | `");
        text.push_str(&port.scope_class);
        text.push_str("` | `");
        text.push_str(&port.multiplicity);
        text.push_str("` | `");
        text.push_str(&port.lifecycle_owner_component_id);
        text.push_str("` | `");
        text.push_str(&port.activation);
        text.push_str("` | `");
        text.push_str(&port.feature_gate);
        text.push_str("` | `");
        text.push_str(&port.required_authority);
        text.push_str("` | `");
        text.push_str(&port.focused_check);
        text.push_str("` |\n");
    }
    text.push_str("\n## Event ownership\n\n");
    text.push_str("| Event | Producer | Consumers | Delivery | Durability | Lifecycle owner | Activation | Cycle policy | Focused check |\n");
    text.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for event in &catalog.events {
        text.push_str("| `");
        text.push_str(&event.event_id);
        text.push_str("` | `");
        text.push_str(&event.producer_component_id);
        text.push_str("` | ");
        text.push_str(code_list(&event.consumer_component_ids).as_str());
        text.push_str(" | `");
        text.push_str(&event.delivery);
        text.push_str("` | `");
        text.push_str(&event.durability);
        text.push_str("` | `");
        text.push_str(&event.lifecycle_owner_component_id);
        text.push_str("` | `");
        text.push_str(&event.activation);
        text.push_str("` | `");
        text.push_str(&event.cycle_policy);
        text.push_str("` | `");
        text.push_str(&event.focused_check);
        text.push_str("` |\n");
    }
    text.push_str("\n## Validation\n\n");
    text.push_str("Run `scripts/cairn/check-component-catalog --root .` to validate components, dependencies, seams, event routes, activation metadata, maintained product-profile binding, safe evidence fields, fixtures, and this projection. The Nix check is `nix build .#checks.x86_64-linux.component-catalog --no-link -L`.\n");
    text
}

fn format_component_row(entry: &ComponentEntry) -> String {
    format!(
        "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
        entry.component_id,
        entry.component_class,
        entry.owner,
        entry.feature_gate,
        entry.required_authority,
        entry.capability_family,
        entry.focused_checks.join(DOC_ROW_CHECK_SEPARATOR),
        entry.public_import
    )
}

fn code_list(values: &[String]) -> String {
    if values.is_empty() {
        "-".to_string()
    } else {
        values
            .iter()
            .map(|value| format!("`{value}`"))
            .collect::<Vec<_>>()
            .join(DOC_LIST_SEPARATOR)
    }
}

fn print_report(catalog: &Catalog, issues: &[Issue]) {
    for issue in issues {
        println!("ERROR {} {} {}", issue.code, issue.subject, issue.detail);
    }

    let mut by_class = BTreeMap::<&str, usize>::new();
    let mut by_family = BTreeMap::<&str, usize>::new();
    let mut dependency_links = 0usize;
    let mut composition_links = 0usize;
    for entry in &catalog.components {
        *by_class.entry(entry.component_class.as_str()).or_default() += 1;
        *by_family
            .entry(entry.capability_family.as_str())
            .or_default() += 1;
        dependency_links += entry.depends_on.len();
        composition_links += entry.composes_with.len();
    }

    for (class, count) in by_class {
        println!("class {class}: {count}");
    }
    for (family, count) in by_family {
        println!("family {family}: {count}");
    }
    println!("dependency links: {dependency_links}");
    println!("composition links: {composition_links}");
    println!("capability seams: {}", catalog.ports.len());
    println!("event routes: {}", catalog.events.len());
    if issues.is_empty() {
        println!("component catalog: ok");
    }
}

fn issue(code: &'static str, subject: &str, detail: &str) -> Issue {
    Issue {
        code,
        subject: subject.to_string(),
        detail: detail.to_string(),
    }
}
