//! The declared checks file: the only commands the agent may run.

// r[impl agent_harness_runtime.acp_coding_agent.checks]

use serde::Deserialize;

use crate::admission::is_check_id;

/// Schema identity of the checks file.
pub const CHECKS_SCHEMA: &str = "animus.acp-coding-agent.checks.v1";
/// Most checks one file may declare.
pub const MAX_CHECKS: usize = 32;
/// Longest admitted check timeout: thirty minutes.
pub const MAX_TIMEOUT_MS: u64 = 30 * 60 * 1000;
/// Largest admitted stdout or stderr capture per check.
pub const MAX_STREAM_BYTES: u64 = 1_048_576;
/// Most argv entries per check.
pub const MAX_ARGV: usize = 64;
/// Longest argv entry, in bytes.
pub const MAX_ARG_BYTES: usize = 4096;
/// Largest checks file, in bytes.
pub const MAX_CHECKS_FILE_BYTES: usize = 262_144;

/// One declared check: an exact argv run in the workspace root with a finite budget.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeclaredCheck {
    /// Check id the model names in `run_check`.
    pub id: String,
    /// Exact argv; the first entry is the program.
    pub argv: Vec<String>,
    /// Wall-clock budget.
    pub timeout_ms: u64,
    /// Captured stdout bound.
    pub stdout_max_bytes: u64,
    /// Captured stderr bound.
    pub stderr_max_bytes: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ChecksFile {
    schema: String,
    checks: Vec<DeclaredCheck>,
}

/// An admitted checks file and its BLAKE3 identity, which enters session receipts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmittedChecks {
    checks: Vec<DeclaredCheck>,
    identity: String,
}

/// Why a checks file was refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChecksIssue {
    /// The file is larger than [`MAX_CHECKS_FILE_BYTES`].
    TooLarge,
    /// The file is not the exact JSON document.
    Malformed,
    /// The schema identity differs.
    Schema,
    /// More than [`MAX_CHECKS`] checks.
    TooMany,
    /// A check id is malformed.
    Id(String),
    /// A check id repeats.
    Duplicate(String),
    /// An argv is empty, too long, or holds an empty, oversized or NUL entry.
    Argv(String),
    /// A budget is zero or above its ceiling.
    Budget(String),
}

impl AdmittedChecks {
    /// No declared checks: `run_check` refuses every id.
    #[must_use]
    pub fn none() -> Self {
        Self {
            checks: Vec::new(),
            identity: blake3::hash(b"").to_hex().to_string(),
        }
    }

    /// The declared check with this id.
    #[must_use]
    pub fn find(&self, id: &str) -> Option<&DeclaredCheck> {
        self.checks.iter().find(|check| check.id == id)
    }

    /// Declared checks, in file order.
    #[must_use]
    pub fn checks(&self) -> &[DeclaredCheck] {
        &self.checks
    }

    /// BLAKE3 hex identity of the exact file bytes.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }
}

fn argv_ok(argv: &[String]) -> bool {
    !argv.is_empty()
        && argv.len() <= MAX_ARGV
        && argv
            .iter()
            .all(|arg| !arg.is_empty() && arg.len() <= MAX_ARG_BYTES && !arg.contains('\0'))
}

fn budget_ok(check: &DeclaredCheck) -> bool {
    let within = |value: u64, ceiling: u64| value > 0 && value <= ceiling;
    within(check.timeout_ms, MAX_TIMEOUT_MS)
        && within(check.stdout_max_bytes, MAX_STREAM_BYTES)
        && within(check.stderr_max_bytes, MAX_STREAM_BYTES)
}

/// Admit a checks file from its exact bytes.
///
/// # Errors
///
/// Returns a [`ChecksIssue`] naming the first refused property.
pub fn parse_checks(bytes: &[u8]) -> Result<AdmittedChecks, ChecksIssue> {
    if bytes.len() > MAX_CHECKS_FILE_BYTES {
        return Err(ChecksIssue::TooLarge);
    }
    let file: ChecksFile = serde_json::from_slice(bytes).map_err(|_| ChecksIssue::Malformed)?;
    if file.schema != CHECKS_SCHEMA {
        return Err(ChecksIssue::Schema);
    }
    if file.checks.len() > MAX_CHECKS {
        return Err(ChecksIssue::TooMany);
    }
    for (index, check) in file.checks.iter().enumerate() {
        if !is_check_id(&check.id) {
            return Err(ChecksIssue::Id(check.id.clone()));
        }
        if file.checks[..index]
            .iter()
            .any(|earlier| earlier.id == check.id)
        {
            return Err(ChecksIssue::Duplicate(check.id.clone()));
        }
        if !argv_ok(&check.argv) {
            return Err(ChecksIssue::Argv(check.id.clone()));
        }
        if !budget_ok(check) {
            return Err(ChecksIssue::Budget(check.id.clone()));
        }
    }
    Ok(AdmittedChecks {
        checks: file.checks,
        identity: blake3::hash(bytes).to_hex().to_string(),
    })
}
