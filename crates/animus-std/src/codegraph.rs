//! Capability-scoped Rust codebase-graph host shell.

use std::io::Read;

use animus_adapters::codegraph::{
    CodeGraphContribution, CodeGraphError, CodeGraphEvent, CodeGraphEventKind, CodeGraphIndex,
    CodeGraphProfile, CodeGraphQuery, CodeGraphQueryResult, CodeGraphQueryService, CodeGraphSymbol,
    CodeGraphSymbolKind, admit_code_graph_file, apply_code_graph_event, build_code_graph_index,
    normalize_code_graph_contribution, query_code_graph,
};
use cap_std::fs::Dir;
use serde::Deserialize;
use tree_sitter::{Node, Parser};

const RUST_LANGUAGE: &str = "rust";
const RUST_PARSER_COHORT: &str = "tree-sitter-rust-0.24";
const PROFILE_SCHEMA: &str = "animus.codebase-graph-profile.v1";
const REQUIRED_STALE_POLICY: &str = "block_until_rescan";
const REQUIRED_BINARY_POLICY: &str = "reject";
const REQUIRED_SPECIAL_FILE_POLICY: &str = "reject";
const REQUIRED_REDACTION: &str = "metadata_only";

/// Host-shell failure with stable admission classes.
#[derive(Debug)]
pub enum CodeGraphHostError {
    /// Pure profile, path, budget, freshness, or event admission failed.
    Admission(CodeGraphError),
    /// A capability-scoped filesystem operation failed.
    FileSystem,
    /// A symlink or unsupported file class was observed.
    UnsupportedFile,
    /// The selected parser cohort could not parse the input.
    Parse,
    /// Generated profile JSON was malformed or semantically incompatible.
    Config,
}

impl From<CodeGraphError> for CodeGraphHostError {
    fn from(value: CodeGraphError) -> Self {
        Self::Admission(value)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedProfile {
    schema: String,
    profile_id: String,
    language: String,
    parser_cohort: String,
    query_cohort: String,
    capability_root_label: String,
    include_extensions: Vec<String>,
    exclude_components: Vec<String>,
    budgets: GeneratedBudgets,
    stale_event_policy: String,
    cache_posture: String,
    binary_policy: String,
    special_file_policy: String,
    redaction: String,
    non_claims: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedBudgets {
    #[serde(rename = "max_files")]
    files: usize,
    #[serde(rename = "max_file_bytes")]
    file_bytes: usize,
    #[serde(rename = "max_results")]
    results: usize,
}

/// Revalidates the checked Nickel projection without runtime Nickel evaluation.
///
/// # Errors
///
/// Returns `Config` for malformed or widened generated policy and `Admission` for invalid core fields.
pub fn parse_code_graph_profile_json(bytes: &[u8]) -> Result<CodeGraphProfile, CodeGraphHostError> {
    let generated: GeneratedProfile =
        serde_json::from_slice(bytes).map_err(|_| CodeGraphHostError::Config)?;
    let valid_policy = generated.schema == PROFILE_SCHEMA
        && generated.language == RUST_LANGUAGE
        && generated.parser_cohort == RUST_PARSER_COHORT
        && generated.include_extensions == ["rs"]
        && generated.exclude_components == [".git", ".jj", "target"]
        && generated.stale_event_policy == REQUIRED_STALE_POLICY
        && matches!(
            generated.cache_posture.as_str(),
            "disabled" | "capability_scoped_disposable"
        )
        && generated.binary_policy == REQUIRED_BINARY_POLICY
        && generated.special_file_policy == REQUIRED_SPECIAL_FILE_POLICY
        && generated.redaction == REQUIRED_REDACTION
        && generated
            .non_claims
            .iter()
            .any(|claim| claim == "not-semantic-analysis")
        && generated
            .non_claims
            .iter()
            .any(|claim| claim == "not-release-eligibility");
    if !valid_policy {
        return Err(CodeGraphHostError::Config);
    }
    animus_adapters::codegraph::validate_code_graph_profile(
        &animus_adapters::codegraph::CodeGraphProfileDraft {
            profile_id: &generated.profile_id,
            language: &generated.language,
            parser_cohort: &generated.parser_cohort,
            query_cohort: &generated.query_cohort,
            root_label: &generated.capability_root_label,
            max_files: generated.budgets.files,
            max_file_bytes: generated.budgets.file_bytes,
            max_results: generated.budgets.results,
        },
    )
    .map_err(Into::into)
}

/// Explicit host event observation.
#[derive(Clone, Copy)]
pub struct CodeGraphHostEvent<'a> {
    /// Event class.
    pub kind: CodeGraphEventKind,
    /// Prior or current relative path.
    pub relative_path: &'a str,
    /// New relative path for rename.
    pub new_relative_path: Option<&'a str>,
}

/// In-memory optional codebase-graph service.
pub struct CodeGraphHost {
    profile: CodeGraphProfile,
    index: CodeGraphIndex,
}

impl CodeGraphHost {
    /// Performs a bounded initial index over caller-selected relative paths.
    ///
    /// # Errors
    ///
    /// Returns a typed host error for unsupported profiles, paths, files, parsing, or budgets.
    pub fn index_rust_files(
        root: &Dir,
        profile: CodeGraphProfile,
        relative_paths: &[String],
    ) -> Result<Self, CodeGraphHostError> {
        if profile.language != RUST_LANGUAGE || profile.parser_cohort != RUST_PARSER_COHORT {
            return Err(CodeGraphHostError::Parse);
        }
        if relative_paths.len() > profile.max_files {
            return Err(CodeGraphHostError::Admission(
                CodeGraphError::BudgetExceeded,
            ));
        }
        let mut ordered_paths = relative_paths.to_vec();
        ordered_paths.sort();
        ordered_paths.dedup();
        if ordered_paths.len() != relative_paths.len() {
            return Err(CodeGraphHostError::Admission(CodeGraphError::InvalidLabel));
        }
        let mut contributions = Vec::with_capacity(ordered_paths.len());
        for relative_path in ordered_paths {
            contributions.push(read_and_parse(root, &profile, &relative_path)?);
        }
        let index = build_code_graph_index(&profile, contributions)?;
        Ok(Self { profile, index })
    }

    /// Applies one already-observed event and optional replacement bytes.
    ///
    /// # Errors
    ///
    /// Returns a typed host error for malformed events, inaccessible replacements, or parsing failures.
    pub fn apply_event(
        &mut self,
        root: &Dir,
        event: CodeGraphHostEvent<'_>,
    ) -> Result<(), CodeGraphHostError> {
        let replacement_path = match event.kind {
            CodeGraphEventKind::Create | CodeGraphEventKind::Modify => Some(event.relative_path),
            CodeGraphEventKind::Rename => event.new_relative_path,
            CodeGraphEventKind::Delete
            | CodeGraphEventKind::Overflow
            | CodeGraphEventKind::Gap
            | CodeGraphEventKind::Unsupported => None,
        };
        let contribution = replacement_path
            .map(|path| read_and_parse(root, &self.profile, path))
            .transpose()?;
        self.index = apply_code_graph_event(
            &self.profile,
            &self.index,
            CodeGraphEvent {
                kind: event.kind,
                relative_path: event.relative_path,
                new_relative_path: event.new_relative_path,
                contribution,
            },
        )?;
        Ok(())
    }

    /// Executes a direct bounded query without cloning the full index.
    ///
    /// # Errors
    ///
    /// Returns a typed host error when the index is stale or the query is invalid.
    pub fn query(
        &self,
        query: &CodeGraphQuery<'_>,
    ) -> Result<CodeGraphQueryResult, CodeGraphHostError> {
        query_code_graph(&self.profile, &self.index, query).map_err(Into::into)
    }

    /// Returns the current derived index by shared reference.
    #[must_use]
    pub const fn index(&self) -> &CodeGraphIndex {
        &self.index
    }
}

impl CodeGraphQueryService for CodeGraphHost {
    fn query_code_graph(
        &self,
        query: &CodeGraphQuery<'_>,
    ) -> Result<CodeGraphQueryResult, CodeGraphError> {
        query_code_graph(&self.profile, &self.index, query)
    }
}

fn read_and_parse(
    root: &Dir,
    profile: &CodeGraphProfile,
    relative_path: &str,
) -> Result<CodeGraphContribution, CodeGraphHostError> {
    let metadata = root
        .symlink_metadata(relative_path)
        .map_err(|_| CodeGraphHostError::FileSystem)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(CodeGraphHostError::UnsupportedFile);
    }
    let measured_bytes =
        usize::try_from(metadata.len()).map_err(|_| CodeGraphHostError::FileSystem)?;
    if measured_bytes > profile.max_file_bytes {
        return Err(CodeGraphHostError::Admission(
            CodeGraphError::BudgetExceeded,
        ));
    }
    let file = root
        .open(relative_path)
        .map_err(|_| CodeGraphHostError::FileSystem)?;
    let mut bytes = Vec::with_capacity(measured_bytes);
    file.take(
        u64::try_from(profile.max_file_bytes).map_err(|_| CodeGraphHostError::FileSystem)? + 1,
    )
    .read_to_end(&mut bytes)
    .map_err(|_| CodeGraphHostError::FileSystem)?;
    parse_rust_contribution(profile, relative_path, &bytes)
}

fn parse_rust_contribution(
    profile: &CodeGraphProfile,
    relative_path: &str,
    bytes: &[u8],
) -> Result<CodeGraphContribution, CodeGraphHostError> {
    let source_identity = admit_code_graph_file(profile, relative_path, bytes)?;
    let text = std::str::from_utf8(bytes).map_err(|_| CodeGraphHostError::Parse)?;
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .map_err(|_| CodeGraphHostError::Parse)?;
    let tree = parser.parse(text, None).ok_or(CodeGraphHostError::Parse)?;
    if tree.root_node().has_error() {
        return Err(CodeGraphHostError::Parse);
    }
    let mut symbols = Vec::new();
    collect_symbols(tree.root_node(), bytes, relative_path, &mut symbols);
    normalize_code_graph_contribution(CodeGraphContribution {
        relative_path: relative_path.to_owned(),
        source_identity,
        symbols,
    })
    .map_err(Into::into)
}

fn collect_symbols(
    node: Node<'_>,
    bytes: &[u8],
    relative_path: &str,
    output: &mut Vec<CodeGraphSymbol>,
) {
    if node.kind() == "function_item" {
        if let Some(name_node) = node.child_by_field_name("name") {
            push_symbol(
                name_node,
                SymbolSource {
                    bytes,
                    relative_path,
                    kind: CodeGraphSymbolKind::Definition,
                },
                output,
            );
        }
    } else if node.kind() == "identifier"
        && node
            .parent()
            .is_none_or(|parent| parent.kind() != "function_item")
    {
        push_symbol(
            node,
            SymbolSource {
                bytes,
                relative_path,
                kind: CodeGraphSymbolKind::Reference,
            },
            output,
        );
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_symbols(child, bytes, relative_path, output);
    }
}

#[derive(Clone, Copy)]
struct SymbolSource<'a> {
    bytes: &'a [u8],
    relative_path: &'a str,
    kind: CodeGraphSymbolKind,
}

fn push_symbol(node: Node<'_>, source: SymbolSource<'_>, output: &mut Vec<CodeGraphSymbol>) {
    if let Ok(name) = node.utf8_text(source.bytes) {
        output.push(CodeGraphSymbol {
            name: name.to_owned(),
            kind: source.kind,
            relative_path: source.relative_path.to_owned(),
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_adapters::codegraph::{CodeGraphProfileDraft, validate_code_graph_profile};

    const FILE_BUDGET: usize = 4;
    const BYTE_BUDGET: usize = 4096;
    const RESULT_BUDGET: usize = 8;

    fn profile() -> CodeGraphProfile {
        validate_code_graph_profile(&CodeGraphProfileDraft {
            profile_id: "rust-first",
            language: RUST_LANGUAGE,
            parser_cohort: RUST_PARSER_COHORT,
            query_cohort: "rust-query-v1",
            root_label: "worktree",
            max_files: FILE_BUDGET,
            max_file_bytes: BYTE_BUDGET,
            max_results: RESULT_BUDGET,
        })
        .expect("valid profile")
    }

    #[test]
    fn positive_generated_profile_is_revalidated() {
        let parsed = parse_code_graph_profile_json(include_bytes!(
            "../../../config/generated/codebase-graph-profile.json"
        ))
        .expect("checked profile");
        assert_eq!(parsed.parser_cohort, RUST_PARSER_COHORT);
    }

    #[test]
    fn negative_generated_profile_rejects_unknown_and_widened_policy() {
        let checked = include_str!("../../../config/generated/codebase-graph-profile.json");
        let widened = checked.replace(
            "\"binary_policy\": \"reject\"",
            "\"binary_policy\": \"allow\"",
        );
        assert!(matches!(
            parse_code_graph_profile_json(widened.as_bytes()),
            Err(CodeGraphHostError::Config)
        ));
        let unknown = checked.replacen('{', "{\"unknown\":true,", 1);
        assert!(matches!(
            parse_code_graph_profile_json(unknown.as_bytes()),
            Err(CodeGraphHostError::Config)
        ));
    }

    #[test]
    fn positive_parser_finds_function_definition_and_reference() {
        let contribution =
            parse_rust_contribution(&profile(), "src/lib.rs", b"fn run() { run(); }")
                .expect("parse");
        assert!(
            contribution.symbols.iter().any(
                |symbol| symbol.name == "run" && symbol.kind == CodeGraphSymbolKind::Definition
            )
        );
        assert!(contribution.symbols.iter().any(|symbol| symbol.name == "run" && symbol.kind == CodeGraphSymbolKind::Reference));
    }

    #[test]
    fn negative_parser_rejects_malformed_binary_and_escaping_paths() {
        assert!(matches!(
            parse_rust_contribution(&profile(), "src/lib.rs", b"fn {"),
            Err(CodeGraphHostError::Parse)
        ));
        assert!(matches!(
            parse_rust_contribution(&profile(), "src/lib.rs", &[0]),
            Err(CodeGraphHostError::Admission(
                CodeGraphError::UnsupportedText
            ))
        ));
        assert!(matches!(
            parse_rust_contribution(&profile(), "../lib.rs", b"fn run() {}"),
            Err(CodeGraphHostError::Admission(CodeGraphError::UnsafePath))
        ));
    }

    #[test]
    fn positive_capability_root_indexes_updates_and_queries() {
        let root_path = test_root("positive");
        std::fs::create_dir_all(root_path.join("src")).expect("create fixture root");
        std::fs::write(root_path.join("src/lib.rs"), "fn run() { run(); }").expect("write fixture");
        let root = Dir::open_ambient_dir(&root_path, cap_std::ambient_authority())
            .expect("open capability root");
        let mut host =
            CodeGraphHost::index_rust_files(&root, profile(), &["src/lib.rs".to_owned()])
                .expect("index");
        let result = host
            .query(&CodeGraphQuery {
                symbol: "run",
                kind: Some(CodeGraphSymbolKind::Definition),
                max_results: 1,
            })
            .expect("query");
        assert_eq!(result.matches.len(), 1);
        std::fs::write(root_path.join("src/lib.rs"), "fn next() {}").expect("modify fixture");
        host.apply_event(
            &root,
            CodeGraphHostEvent {
                kind: CodeGraphEventKind::Modify,
                relative_path: "src/lib.rs",
                new_relative_path: None,
            },
        )
        .expect("update");
        let result = host
            .query(&CodeGraphQuery {
                symbol: "next",
                kind: Some(CodeGraphSymbolKind::Definition),
                max_results: 1,
            })
            .expect("updated query");
        assert_eq!(result.matches.len(), 1);
        std::fs::remove_dir_all(root_path).expect("cleanup fixture");
    }

    #[cfg(unix)]
    #[test]
    fn negative_capability_root_rejects_symlink_special_and_oversized_files() {
        use std::os::unix::fs::symlink;

        let root_path = test_root("negative");
        std::fs::create_dir_all(root_path.join("src")).expect("create fixture root");
        std::fs::write(root_path.join("outside.rs"), "fn outside() {}")
            .expect("write outside fixture");
        symlink(root_path.join("outside.rs"), root_path.join("src/link.rs"))
            .expect("create symlink");
        std::fs::write(root_path.join("src/large.rs"), vec![b'x'; BYTE_BUDGET + 1])
            .expect("write large fixture");
        let root = Dir::open_ambient_dir(&root_path, cap_std::ambient_authority())
            .expect("open capability root");
        assert!(matches!(
            CodeGraphHost::index_rust_files(&root, profile(), &["src/link.rs".to_owned()]),
            Err(CodeGraphHostError::UnsupportedFile)
        ));
        assert!(matches!(
            CodeGraphHost::index_rust_files(&root, profile(), &["src/large.rs".to_owned()]),
            Err(CodeGraphHostError::Admission(
                CodeGraphError::BudgetExceeded
            ))
        ));
        std::fs::remove_dir_all(root_path).expect("cleanup fixture");
    }

    fn test_root(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("animus-codegraph-{label}-{}", std::process::id()))
    }
}
