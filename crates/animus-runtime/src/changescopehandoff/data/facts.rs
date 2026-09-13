//! Borrowed scope drafts and admitted immutable scope facts.

/// One borrowed declared hunk inside a proposed change scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChangeScopeHunkDraft<'input> {
    /// Declared file path label that owns this hunk.
    pub file_path: &'input str,
    /// Immutable BLAKE3 hunk-content identity.
    pub hunk_identity: &'input str,
    /// Declared old-file start line.
    pub old_start: u32,
    /// Declared old-file line count.
    pub old_lines: u32,
    /// Declared new-file start line.
    pub new_start: u32,
    /// Declared new-file line count.
    pub new_lines: u32,
}

/// Borrowed complete facts for one proposed change scope.
#[derive(Clone, Copy, Debug)]
pub struct ChangeScopeDraft<'input> {
    /// Bounded driving-task label that owns the proposed change.
    pub driving_task: &'input str,
    /// Declared bounded file-path labels.
    pub files: &'input [&'input str],
    /// Declared bounded hunks over `files`.
    pub hunks: &'input [ChangeScopeHunkDraft<'input>],
    /// Declared bounded validation-obligation labels.
    pub obligations: &'input [&'input str],
}

/// One validated declared hunk.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeScopeHunk {
    pub(in crate::changescopehandoff) file_path: String,
    pub(in crate::changescopehandoff) hunk_identity: String,
    pub(in crate::changescopehandoff) old_start: u32,
    pub(in crate::changescopehandoff) old_lines: u32,
    pub(in crate::changescopehandoff) new_start: u32,
    pub(in crate::changescopehandoff) new_lines: u32,
}

impl ChangeScopeHunk {
    /// Returns the declared owning file-path label.
    #[must_use]
    pub fn file_path(&self) -> &str {
        self.file_path.as_str()
    }

    /// Returns the immutable BLAKE3 hunk-content identity.
    #[must_use]
    pub fn hunk_identity(&self) -> &str {
        self.hunk_identity.as_str()
    }

    /// Returns the declared old-file start line.
    #[must_use]
    pub const fn old_start(&self) -> u32 {
        self.old_start
    }

    /// Returns the declared old-file line count.
    #[must_use]
    pub const fn old_lines(&self) -> u32 {
        self.old_lines
    }

    /// Returns the declared new-file start line.
    #[must_use]
    pub const fn new_start(&self) -> u32 {
        self.new_start
    }

    /// Returns the declared new-file line count.
    #[must_use]
    pub const fn new_lines(&self) -> u32 {
        self.new_lines
    }
}

/// One typed immutable change scope binding files, hunks, task, and obligations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeScope {
    pub(in crate::changescopehandoff) scope_identity: String,
    pub(in crate::changescopehandoff) driving_task: String,
    pub(in crate::changescopehandoff) files: Vec<String>,
    pub(in crate::changescopehandoff) hunks: Vec<ChangeScopeHunk>,
    pub(in crate::changescopehandoff) obligations: Vec<String>,
}

impl ChangeScope {
    /// Returns the immutable BLAKE3 scope identity.
    #[must_use]
    pub fn scope_identity(&self) -> &str {
        self.scope_identity.as_str()
    }

    /// Returns the bounded driving-task label.
    #[must_use]
    pub fn driving_task(&self) -> &str {
        self.driving_task.as_str()
    }

    /// Returns the declared bounded file-path labels.
    #[must_use]
    pub fn files(&self) -> &[String] {
        self.files.as_slice()
    }

    /// Returns the declared bounded hunks.
    #[must_use]
    pub fn hunks(&self) -> &[ChangeScopeHunk] {
        self.hunks.as_slice()
    }

    /// Returns the declared bounded validation-obligation labels.
    #[must_use]
    pub fn obligations(&self) -> &[String] {
        self.obligations.as_slice()
    }
}
