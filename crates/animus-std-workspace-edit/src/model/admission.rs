/// Canonical host snapshot admitted by pure validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedSnapshot {
    pub(crate) root_identity: String,
    pub(crate) resource_identity: String,
    pub(crate) revision_identity: String,
    pub(crate) metadata_policy_identity: String,
    pub(crate) file_metadata_identity: String,
    pub(crate) writer_generation: u64,
    pub(crate) source_bytes: Vec<u8>,
    pub(crate) source_byte_count: u64,
    pub(crate) content_identity: String,
    pub(crate) base_identity: String,
    pub(crate) line_ending_mode: crate::model::LineEndingMode,
    pub(crate) final_newline: crate::model::FinalNewlineState,
    pub(crate) replacement_ending: crate::model::ReplacementEnding,
    pub(crate) lines: Vec<crate::model::text::SourceLine>,
    pub(crate) tags: Vec<(String, u32)>,
    pub(crate) visible_ranges: Vec<crate::model::text::VisibleRange>,
    pub(crate) visible_boundaries: Vec<crate::model::text::VisibleBoundary>,
    pub(crate) end_boundary_visible: bool,
    pub(crate) visibility_identity: String,
    pub(crate) identity: String,
}

impl AdmittedSnapshot {
    /// Return the capability-root identity.
    #[must_use]
    pub fn root_identity(&self) -> &str {
        &self.root_identity
    }

    /// Return the logical resource identity.
    #[must_use]
    pub fn resource_identity(&self) -> &str {
        &self.resource_identity
    }

    /// Return the revision identity.
    #[must_use]
    pub fn revision_identity(&self) -> &str {
        &self.revision_identity
    }

    /// Return the metadata-policy identity.
    #[must_use]
    pub fn metadata_policy_identity(&self) -> &str {
        &self.metadata_policy_identity
    }

    /// Return the observed file-metadata identity.
    #[must_use]
    pub fn file_metadata_identity(&self) -> &str {
        &self.file_metadata_identity
    }

    /// Return the current Animus writer generation.
    #[must_use]
    pub const fn writer_generation(&self) -> u64 {
        self.writer_generation
    }

    /// Return the complete before-content BLAKE3 identity.
    #[must_use]
    pub fn content_identity(&self) -> &str {
        &self.content_identity
    }

    /// Return the visible-span and boundary identity.
    #[must_use]
    pub fn visibility_identity(&self) -> &str {
        &self.visibility_identity
    }

    /// Return the canonical snapshot identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// Return the exact line-ending mode.
    #[must_use]
    pub const fn line_ending_mode(&self) -> crate::model::LineEndingMode {
        self.line_ending_mode
    }

    /// Return the exact final-newline state.
    #[must_use]
    pub const fn final_newline(&self) -> crate::model::FinalNewlineState {
        self.final_newline
    }

    /// Return the admitted source byte count.
    #[must_use]
    pub const fn source_bytes_len(&self) -> u64 {
        self.source_byte_count
    }
}
