use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

const SUCCESS_EXIT: i32 = 0;
const FAILURE_EXIT: i32 = 1;
const ROOT_FLAG: &str = "--root";
const CHECK_LABEL: &str = "adapter maintainability map";
const DOC_RELATIVE_PATH: &str = "docs/adapter-module-ownership.md";

const MISSING_ARGUMENT: &str = "MISSING_ARGUMENT";
const UNKNOWN_ARGUMENT: &str = "UNKNOWN_ARGUMENT";
const MISSING_DOC: &str = "ADAPTER_MAINTAINABILITY_MISSING_DOC";
const MISSING_SECTION: &str = "ADAPTER_MAINTAINABILITY_MISSING_SECTION";
const MISSING_REQUIREMENT: &str = "ADAPTER_MAINTAINABILITY_MISSING_REQUIREMENT";
const MISSING_SEMANTIC_SHARD: &str = "ADAPTER_MAINTAINABILITY_MISSING_SEMANTIC_SHARD";
const MECHANICAL_SHARD: &str = "ADAPTER_MAINTAINABILITY_MECHANICAL_SHARD";
const MISSING_WRAPPER: &str = "ADAPTER_MAINTAINABILITY_MISSING_WRAPPER";
const STALE_INCLUDE: &str = "ADAPTER_MAINTAINABILITY_STALE_INCLUDE";
const UNDOCUMENTED_SHARD: &str = "ADAPTER_MAINTAINABILITY_UNDOCUMENTED_SHARD";

const REQUIRED_SECTIONS: &[&str] = &[
    "## Hotspot inventory",
    "## Semantic ownership map",
    "## Compatibility aliases and migrations",
    "## Functional-core / shell split",
    "## Lint allowance scope",
    "## Verification and non-claims",
];

const REQUIRED_REQUIREMENTS: &[&str] = &[
    "r[agent_harness_runtime.adapter_crate_maintainability.inventory]",
    "r[agent_harness_runtime.adapter_crate_maintainability.ownership_map]",
    "r[agent_harness_runtime.adapter_crate_maintainability.semantic_shards]",
    "r[agent_harness_runtime.adapter_crate_maintainability.functional_core]",
    "r[agent_harness_runtime.adapter_crate_maintainability.lint_scope]",
    "r[agent_harness_runtime.adapter_crate_maintainability.verification]",
];

const SEMANTIC_SHARDS: &[&str] = &[
    "crates/animus-adapters/src/front/memory/impl/generic/adapter_records.rs",
    "crates/animus-adapters/src/front/memory/impl/generic/operation_controls.rs",
    "crates/animus-adapters/src/front/memory/impl/generic/conformance_matrix.rs",
    "crates/animus-adapters/src/front/tool/gateway/delivery_types.rs",
    "crates/animus-adapters/src/front/tool/gateway/delivery_request.rs",
    "crates/animus-adapters/src/front/tool/gateway/delivery_policy.rs",
    "crates/animus-adapters/src/front/tool/gateway/delivery_outbox.rs",
    "crates/animus-adapters/src/front/tool/gateway/delivery_retry_receipts.rs",
    "crates/animus-adapters/src/front/tool/gateway/delivery_errors.rs",
    "crates/animus-adapters/src/review/impl/report_types.rs",
    "crates/animus-adapters/src/review/impl/metric_records.rs",
    "crates/animus-adapters/src/review/impl/report_reducers.rs",
    "crates/animus-adapters/src/review/impl/report_parser.rs",
    "crates/animus-adapters/src/review/impl/corrective_memory.rs",
    "crates/animus-adapters/src/review/impl/lifecycle_review.rs",
];

const MECHANICAL_SHARDS: &[&str] = &[
    "crates/animus-adapters/src/front/memory/impl/generic/impl009.rs",
    "crates/animus-adapters/src/front/memory/impl/generic/impl010.rs",
    "crates/animus-adapters/src/front/memory/impl/generic/impl011.rs",
    "crates/animus-adapters/src/front/tool/gateway/impl001.rs",
    "crates/animus-adapters/src/front/tool/gateway/impl002.rs",
    "crates/animus-adapters/src/front/tool/gateway/impl003.rs",
    "crates/animus-adapters/src/front/tool/gateway/impl004.rs",
    "crates/animus-adapters/src/front/tool/gateway/impl005.rs",
    "crates/animus-adapters/src/front/tool/gateway/impl006.rs",
    "crates/animus-adapters/src/review/impl/impl001.rs",
    "crates/animus-adapters/src/review/impl/impl002.rs",
    "crates/animus-adapters/src/review/impl/impl003.rs",
    "crates/animus-adapters/src/review/impl/impl004.rs",
    "crates/animus-adapters/src/review/impl/impl006.rs",
    "crates/animus-adapters/src/review/impl/impl007.rs",
];

const MEMORY_WRAPPER_REQUIRED: &[&str] = &[
    "memory/impl/generic/adapter_records.rs",
    "memory/impl/generic/operation_controls.rs",
    "memory/impl/generic/conformance_matrix.rs",
];
const MEMORY_WRAPPER_FORBIDDEN: &[&str] = &[
    "memory/impl/generic/impl009.rs",
    "memory/impl/generic/impl010.rs",
    "memory/impl/generic/impl011.rs",
];
const GATEWAY_WRAPPER_REQUIRED: &[&str] = &[
    "gateway/delivery_types.rs",
    "gateway/delivery_request.rs",
    "gateway/delivery_policy.rs",
    "gateway/delivery_outbox.rs",
    "gateway/delivery_retry_receipts.rs",
    "gateway/delivery_errors.rs",
];
const GATEWAY_WRAPPER_FORBIDDEN: &[&str] = &[
    "gateway/impl001.rs",
    "gateway/impl002.rs",
    "gateway/impl003.rs",
    "gateway/impl004.rs",
    "gateway/impl005.rs",
    "gateway/impl006.rs",
];
const REVIEW_WRAPPER_REQUIRED: &[&str] = &[
    "review/impl/report_types.rs",
    "review/impl/metric_records.rs",
    "review/impl/report_reducers.rs",
    "review/impl/report_parser.rs",
    "review/impl/corrective_memory.rs",
    "review/impl/lifecycle_review.rs",
];
const REVIEW_WRAPPER_FORBIDDEN: &[&str] = &[
    "review/impl/impl001.rs",
    "review/impl/impl002.rs",
    "review/impl/impl003.rs",
    "review/impl/impl004.rs",
    "review/impl/impl006.rs",
    "review/impl/impl007.rs",
];

struct WrapperExpectation {
    path: &'static str,
    required: &'static [&'static str],
    forbidden: &'static [&'static str],
}

const WRAPPER_EXPECTATIONS: &[WrapperExpectation] = &[
    WrapperExpectation {
        path: "crates/animus-adapters/src/front/memory.rs",
        required: MEMORY_WRAPPER_REQUIRED,
        forbidden: MEMORY_WRAPPER_FORBIDDEN,
    },
    WrapperExpectation {
        path: "crates/animus-adapters/src/front/tool/gateway.rs",
        required: GATEWAY_WRAPPER_REQUIRED,
        forbidden: GATEWAY_WRAPPER_FORBIDDEN,
    },
    WrapperExpectation {
        path: "crates/animus-adapters/src/review.rs",
        required: REVIEW_WRAPPER_REQUIRED,
        forbidden: REVIEW_WRAPPER_FORBIDDEN,
    },
];

#[derive(Debug)]
struct Config {
    root: PathBuf,
}

#[derive(Debug)]
struct Issue {
    code: &'static str,
    detail: String,
}

impl Issue {
    fn new(code: &'static str, detail: impl Into<String>) -> Self {
        Self {
            code,
            detail: detail.into(),
        }
    }
}

fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Config, Issue> {
    let mut root = PathBuf::from(".");
    let mut iterator = args.into_iter();
    let _program = iterator.next();
    while let Some(arg) = iterator.next() {
        if arg == ROOT_FLAG {
            let Some(value) = iterator.next() else {
                return Err(Issue::new(MISSING_ARGUMENT, ROOT_FLAG));
            };
            root = PathBuf::from(value);
            continue;
        }
        return Err(Issue::new(UNKNOWN_ARGUMENT, arg));
    }
    Ok(Config { root })
}

fn read_required_file(
    root: &Path,
    relative_path: &str,
    issue_code: &'static str,
) -> Result<String, Issue> {
    let path = root.join(relative_path);
    fs::read_to_string(&path)
        .map_err(|error| Issue::new(issue_code, format!("{}: {}", relative_path, error)))
}

fn require_doc_contains(doc: &str, needle: &str, issue_code: &'static str) -> Result<(), Issue> {
    if doc.contains(needle) {
        return Ok(());
    }
    Err(Issue::new(issue_code, needle))
}

fn path_exists(root: &Path, relative_path: &str) -> bool {
    root.join(relative_path).exists()
}

fn validate_doc(root: &Path) -> Result<String, Issue> {
    let doc = read_required_file(root, DOC_RELATIVE_PATH, MISSING_DOC)?;
    for section in REQUIRED_SECTIONS {
        require_doc_contains(&doc, section, MISSING_SECTION)?;
    }
    for requirement in REQUIRED_REQUIREMENTS {
        require_doc_contains(&doc, requirement, MISSING_REQUIREMENT)?;
    }
    for semantic_shard in SEMANTIC_SHARDS {
        require_doc_contains(&doc, semantic_shard, UNDOCUMENTED_SHARD)?;
    }
    Ok(doc)
}

fn validate_shard_paths(root: &Path) -> Result<(), Issue> {
    for semantic_shard in SEMANTIC_SHARDS {
        if !path_exists(root, semantic_shard) {
            return Err(Issue::new(MISSING_SEMANTIC_SHARD, *semantic_shard));
        }
    }
    for mechanical_shard in MECHANICAL_SHARDS {
        if path_exists(root, mechanical_shard) {
            return Err(Issue::new(MECHANICAL_SHARD, *mechanical_shard));
        }
    }
    Ok(())
}

fn validate_wrapper(root: &Path, expectation: &WrapperExpectation) -> Result<(), Issue> {
    let source = read_required_file(root, expectation.path, MISSING_WRAPPER)?;
    for required in expectation.required {
        if !source.contains(required) {
            return Err(Issue::new(STALE_INCLUDE, *required));
        }
    }
    for forbidden in expectation.forbidden {
        if source.contains(forbidden) {
            return Err(Issue::new(STALE_INCLUDE, *forbidden));
        }
    }
    Ok(())
}

fn validate_wrappers(root: &Path) -> Result<(), Issue> {
    for expectation in WRAPPER_EXPECTATIONS {
        validate_wrapper(root, expectation)?;
    }
    Ok(())
}

fn validate(root: &Path) -> Result<(), Issue> {
    let _doc = validate_doc(root)?;
    validate_shard_paths(root)?;
    validate_wrappers(root)
}

fn run(config: &Config) -> Result<(), Issue> {
    validate(&config.root)
}

fn main() {
    let config = match parse_args(env::args()) {
        Ok(config) => config,
        Err(issue) => {
            eprintln!("ERROR {} {}", issue.code, issue.detail);
            process::exit(FAILURE_EXIT);
        }
    };

    match run(&config) {
        Ok(()) => {
            println!("{}: ok", CHECK_LABEL);
            process::exit(SUCCESS_EXIT);
        }
        Err(issue) => {
            eprintln!("ERROR {} {}", issue.code, issue.detail);
            process::exit(FAILURE_EXIT);
        }
    }
}
