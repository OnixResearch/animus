//! Pure admission of workspace tool requests.

// r[impl agent_harness_runtime.acp_coding_agent.tools]

use serde::Deserialize;

/// Longest admitted relative path, in bytes.
pub const MAX_PATH_BYTES: usize = 1024;
/// Most bytes one `read_file` returns.
pub const MAX_READ_BYTES: usize = 262_144;
/// Most entries one `list_files` returns.
pub const MAX_LIST_ENTRIES: usize = 1024;
/// Longest `old` or `new` text in one edit, in bytes.
pub const MAX_EDIT_TEXT_BYTES: usize = 65_536;
/// Most edits in one `edit_file` request.
pub const MAX_EDITS_PER_CALL: usize = 16;
/// Largest file an edit may produce, in bytes.
pub const MAX_EDITED_FILE_BYTES: usize = 1_048_576;
/// Longest check id, in bytes.
pub const MAX_CHECK_ID_BYTES: usize = 64;
/// Longest tool-call argument document, in bytes.
pub const MAX_ARGUMENT_BYTES: usize = 2 * MAX_EDITS_PER_CALL * MAX_EDIT_TEXT_BYTES + 4096;

/// The tool operations, in a stable order.
pub const TOOL_NAMES: [&str; 4] = ["read_file", "list_files", "edit_file", "run_check"];

/// The one function offered to the model. `animus-std-openai` admits a single tool
/// per cohort, so the four operations share one strict schema as an `anyOf` over a
/// `request` object discriminated by `op`.
pub const WORKSPACE_TOOL_NAME: &str = "workspace";

/// Strict schema for [`WORKSPACE_TOOL_NAME`]: every object lists all its properties
/// as required and forbids others, as `OpenAI` strict mode and `animus-std-openai`
/// require.
pub const WORKSPACE_TOOL_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "request": {
      "anyOf": [
        {"type": "object", "properties": {"op": {"type": "string", "enum": ["read_file"]}, "path": {"type": "string"}}, "required": ["op", "path"], "additionalProperties": false},
        {"type": "object", "properties": {"op": {"type": "string", "enum": ["list_files"]}, "path": {"type": "string"}}, "required": ["op", "path"], "additionalProperties": false},
        {"type": "object", "properties": {"op": {"type": "string", "enum": ["edit_file"]}, "path": {"type": "string"}, "edits": {"type": "array", "items": {"type": "object", "properties": {"old": {"type": "string"}, "new": {"type": "string"}}, "required": ["old", "new"], "additionalProperties": false}}}, "required": ["op", "path", "edits"], "additionalProperties": false},
        {"type": "object", "properties": {"op": {"type": "string", "enum": ["run_check"]}, "check_id": {"type": "string"}}, "required": ["op", "check_id"], "additionalProperties": false}
      ]
    }
  },
  "required": ["request"],
  "additionalProperties": false
}"#;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkspaceArguments {
    request: serde_json::Map<String, serde_json::Value>,
}

/// Admit one call of the single [`WORKSPACE_TOOL_NAME`] function by routing its
/// `request.op` to [`parse_tool_call`].
///
/// # Errors
///
/// Returns a [`ToolIssue`] for malformed arguments, an unknown `op`, or any refusal
/// [`parse_tool_call`] makes.
pub fn parse_workspace_call(json: &str) -> Result<ToolCall, ToolIssue> {
    if json.len() > MAX_ARGUMENT_BYTES {
        return Err(ToolIssue::ArgumentsTooLarge);
    }
    let WorkspaceArguments { mut request } = arguments(json)?;
    let Some(serde_json::Value::String(op)) = request.remove("op") else {
        return Err(ToolIssue::MalformedArguments);
    };
    if !TOOL_NAMES.contains(&op.as_str()) {
        return Err(ToolIssue::UnknownTool);
    }
    let inner = serde_json::Value::Object(request).to_string();
    parse_tool_call(&op, &inner)
}

/// A normalized path relative to the workspace root.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkspacePath {
    text: String,
}

/// Why a path was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathIssue {
    /// The path is empty.
    Empty,
    /// The path is longer than [`MAX_PATH_BYTES`].
    TooLong,
    /// The path starts at the filesystem root.
    Absolute,
    /// The path holds a NUL byte or a backslash.
    ForbiddenCharacter,
    /// The path has an empty, `.` or `..` component.
    ForbiddenComponent,
}

impl WorkspacePath {
    /// Admit a relative path. `"."` names the root itself, for listings only.
    ///
    /// # Errors
    ///
    /// Returns a [`PathIssue`] for any path that could leave or misname the root.
    pub fn parse(text: &str) -> Result<Self, PathIssue> {
        if text.is_empty() {
            return Err(PathIssue::Empty);
        }
        if text.len() > MAX_PATH_BYTES {
            return Err(PathIssue::TooLong);
        }
        if text.starts_with('/') {
            return Err(PathIssue::Absolute);
        }
        if text.contains('\0') || text.contains('\\') {
            return Err(PathIssue::ForbiddenCharacter);
        }
        if text == "." {
            return Ok(Self {
                text: String::new(),
            });
        }
        let forbidden = text
            .split('/')
            .any(|component| component.is_empty() || component == "." || component == "..");
        if forbidden {
            return Err(PathIssue::ForbiddenComponent);
        }
        Ok(Self {
            text: text.to_owned(),
        })
    }

    /// Whether this path names the workspace root.
    #[must_use]
    pub fn is_root(&self) -> bool {
        self.text.is_empty()
    }

    /// The normalized path text; empty for the root.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Path components from the root, for capability-relative traversal.
    pub fn components(&self) -> impl Iterator<Item = &str> {
        self.text
            .split('/')
            .filter(|component| !component.is_empty())
    }
}

/// One exact replacement. An empty `old` creates a file that does not exist.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Edit {
    /// Text that must occur exactly once in the current file.
    pub old: String,
    /// Replacement text.
    pub new: String,
}

/// An admitted tool request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToolCall {
    /// Read one file, up to [`MAX_READ_BYTES`].
    ReadFile {
        /// File to read.
        path: WorkspacePath,
    },
    /// List one directory, up to [`MAX_LIST_ENTRIES`].
    ListFiles {
        /// Directory to list; the root is allowed.
        path: WorkspacePath,
    },
    /// Apply exact replacements to one file, in order.
    EditFile {
        /// File to edit or create.
        path: WorkspacePath,
        /// Replacements, applied in order.
        edits: Vec<Edit>,
    },
    /// Run one declared check by id.
    RunCheck {
        /// Declared check id.
        check_id: String,
    },
}

/// Why a tool request was refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToolIssue {
    /// The tool name is not offered.
    UnknownTool,
    /// The argument document is larger than [`MAX_ARGUMENT_BYTES`].
    ArgumentsTooLarge,
    /// The arguments are not the tool's exact JSON object.
    MalformedArguments,
    /// A path was refused.
    Path(PathIssue),
    /// A path that must name a file names the root.
    RootIsNotAFile,
    /// An edit request is empty, too long, or holds oversized text.
    EditBounds,
    /// A check id is malformed.
    CheckId,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PathArguments {
    path: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EditArguments {
    path: String,
    edits: Vec<Edit>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckArguments {
    check_id: String,
}

fn arguments<T: for<'de> Deserialize<'de>>(json: &str) -> Result<T, ToolIssue> {
    serde_json::from_str(json).map_err(|_| ToolIssue::MalformedArguments)
}

fn file_path(text: &str) -> Result<WorkspacePath, ToolIssue> {
    let path = WorkspacePath::parse(text).map_err(ToolIssue::Path)?;
    if path.is_root() {
        return Err(ToolIssue::RootIsNotAFile);
    }
    Ok(path)
}

/// Whether `id` is a well-formed check id: `^[a-z0-9][a-z0-9-]{0,63}$`.
#[must_use]
pub fn is_check_id(id: &str) -> bool {
    let mut bytes = id.bytes();
    let first_ok = bytes
        .next()
        .is_some_and(|b| b.is_ascii_lowercase() || b.is_ascii_digit());
    first_ok
        && id.len() <= MAX_CHECK_ID_BYTES
        && bytes.all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// Admit one tool call from its name and JSON arguments.
///
/// # Errors
///
/// Returns a [`ToolIssue`] for an unknown tool, malformed or oversized arguments,
/// a refused path, out-of-bounds edits, or a malformed check id.
pub fn parse_tool_call(name: &str, json: &str) -> Result<ToolCall, ToolIssue> {
    if json.len() > MAX_ARGUMENT_BYTES {
        return Err(ToolIssue::ArgumentsTooLarge);
    }
    match name {
        "read_file" => {
            let args: PathArguments = arguments(json)?;
            Ok(ToolCall::ReadFile {
                path: file_path(&args.path)?,
            })
        }
        "list_files" => {
            let args: PathArguments = arguments(json)?;
            Ok(ToolCall::ListFiles {
                path: WorkspacePath::parse(&args.path).map_err(ToolIssue::Path)?,
            })
        }
        "edit_file" => {
            let args: EditArguments = arguments(json)?;
            let bounded = !args.edits.is_empty()
                && args.edits.len() <= MAX_EDITS_PER_CALL
                && args.edits.iter().all(|e| {
                    e.old.len() <= MAX_EDIT_TEXT_BYTES && e.new.len() <= MAX_EDIT_TEXT_BYTES
                });
            if !bounded {
                return Err(ToolIssue::EditBounds);
            }
            Ok(ToolCall::EditFile {
                path: file_path(&args.path)?,
                edits: args.edits,
            })
        }
        "run_check" => {
            let args: CheckArguments = arguments(json)?;
            if !is_check_id(&args.check_id) {
                return Err(ToolIssue::CheckId);
            }
            Ok(ToolCall::RunCheck {
                check_id: args.check_id,
            })
        }
        _ => Err(ToolIssue::UnknownTool),
    }
}

/// Why a set of edits could not be applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditIssue {
    /// Edit number `index` has an `old` text that does not occur.
    NotFound {
        /// Zero-based edit index.
        index: usize,
    },
    /// Edit number `index` has an `old` text that occurs more than once.
    Ambiguous {
        /// Zero-based edit index.
        index: usize,
    },
    /// Creation (an empty `old`) was asked for a file that exists, or combined
    /// with other edits.
    CreateConflict,
    /// An empty `old` was given for an existing file.
    EmptyOld {
        /// Zero-based edit index.
        index: usize,
    },
    /// A non-creating edit names a file that does not exist.
    Missing,
    /// The result would exceed [`MAX_EDITED_FILE_BYTES`].
    TooLarge,
}

/// Compute a file's new text from its current text (`None` when absent) and the
/// admitted edits. Each `old` must occur exactly once in the text produced by the
/// edits before it. The shell publishes the result through the workspace-edit host.
///
/// # Errors
///
/// Returns an [`EditIssue`]; on any issue nothing is to be written.
pub fn apply_edits(current: Option<&str>, edits: &[Edit]) -> Result<String, EditIssue> {
    let creates = edits.first().is_some_and(|e| e.old.is_empty());
    let text = match (current, creates) {
        (None, true) => {
            if edits.len() != 1 {
                return Err(EditIssue::CreateConflict);
            }
            edits[0].new.clone()
        }
        (Some(_), true) => return Err(EditIssue::CreateConflict),
        (None, false) => return Err(EditIssue::Missing),
        (Some(current), false) => {
            let mut text = current.to_owned();
            for (index, edit) in edits.iter().enumerate() {
                if edit.old.is_empty() {
                    return Err(EditIssue::EmptyOld { index });
                }
                let mut found = text.match_indices(edit.old.as_str());
                let Some((at, _)) = found.next() else {
                    return Err(EditIssue::NotFound { index });
                };
                if found.next().is_some() {
                    return Err(EditIssue::Ambiguous { index });
                }
                text.replace_range(at..at + edit.old.len(), &edit.new);
            }
            text
        }
    };
    if text.len() > MAX_EDITED_FILE_BYTES {
        return Err(EditIssue::TooLarge);
    }
    Ok(text)
}
