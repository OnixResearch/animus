/// Maximum admitted source bytes for one workspace edit operation.
pub const MAX_SOURCE_BYTES: usize = 1_048_576;
pub(crate) const MAX_SOURCE_BYTES_U64: u64 = 1_048_576;
/// Maximum operation count for one workspace edit plan.
pub const MAX_EDIT_OPERATIONS: usize = 64;
/// Maximum replacement bytes across one operation.
pub const MAX_REPLACEMENT_BYTES: usize = 65_536;
/// Maximum visible spans or boundaries in one snapshot.
pub const MAX_VISIBILITY_ITEMS: usize = 512;

/// Source line-ending mode observed from exact bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineEndingMode {
    /// Every terminated line uses LF.
    Lf,
    /// Every terminated line uses CRLF.
    CrLf,
    /// Both LF and CRLF occur.
    Mixed,
}

impl LineEndingMode {
    /// Return the stable evidence label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lf => "lf",
            Self::CrLf => "crlf",
            Self::Mixed => "mixed",
        }
    }
}

/// Whether the exact source ends with a line terminator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FinalNewlineState {
    /// The final source line has a terminator.
    Present,
    /// The final source line has no terminator.
    Absent,
}

impl FinalNewlineState {
    /// Return the stable evidence label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Absent => "absent",
        }
    }
}

/// Explicit ending used for newly inserted logical line boundaries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplacementEnding {
    /// Use LF for new boundaries.
    Lf,
    /// Use CRLF for new boundaries.
    CrLf,
}

impl ReplacementEnding {
    pub(crate) const fn bytes(self) -> &'static [u8] {
        match self {
            Self::Lf => b"\n",
            Self::CrLf => b"\r\n",
        }
    }

    /// Return the stable evidence label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lf => "lf",
            Self::CrLf => "crlf",
        }
    }
}

/// Side of one visible tagged line admitted for insertion.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BoundarySide {
    /// Boundary immediately before the tagged line.
    Before,
    /// Boundary immediately after the tagged line.
    After,
}

impl BoundarySide {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Before => "before",
            Self::After => "after",
        }
    }
}

/// Borrowed contiguous visible tagged span.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VisibleSpanDraft<'a> {
    /// First visible tag.
    pub start_tag: &'a str,
    /// Last visible tag.
    pub end_tag: &'a str,
}

/// Borrowed visible insertion boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VisibleBoundaryDraft<'a> {
    /// Adjacent visible line tag.
    pub line_tag: &'a str,
    /// Visible side of the tagged line.
    pub side: BoundarySide,
}

/// Borrowed host snapshot extension over the existing hashed-edit snapshot.
#[derive(Clone, Copy, Debug)]
pub struct SnapshotDraft<'a> {
    /// Existing complete-content and tagged-line snapshot.
    pub base: &'a animus_adapters::tool::hashed::Snapshot,
    /// BLAKE3 identity of the caller-opened capability root.
    pub root_identity: &'a str,
    /// BLAKE3 identity of the logical relative resource.
    pub resource_identity: &'a str,
    /// Exact supplied revision identity.
    pub revision_identity: &'a str,
    /// BLAKE3 identity of the active metadata and file policy.
    pub metadata_policy_identity: &'a str,
    /// BLAKE3 identity of the observed file kind, metadata, owner, and size.
    pub file_metadata_identity: &'a str,
    /// Current Animus-owned writer generation.
    pub writer_generation: u64,
    /// Exact source bytes from the capability-relative first read.
    pub source_bytes: &'a [u8],
    /// Declared line-ending mode.
    pub line_ending_mode: LineEndingMode,
    /// Declared final-newline state.
    pub final_newline: FinalNewlineState,
    /// Explicit ending for new logical boundaries.
    pub replacement_ending: ReplacementEnding,
    /// Whether mixed source endings are explicitly admitted.
    pub mixed_endings_admitted: bool,
    /// Contiguous line ranges visible under this revision.
    pub visible_spans: &'a [VisibleSpanDraft<'a>],
    /// Explicit insertion boundaries visible under this revision.
    pub visible_boundaries: &'a [VisibleBoundaryDraft<'a>],
    /// Whether the caller observed the end-of-file insertion boundary.
    pub end_boundary_visible: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SourceLine {
    pub(crate) start: usize,
    pub(crate) body_end: usize,
    pub(crate) end: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct VisibleRange {
    pub(crate) lines: std::ops::Range<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct VisibleBoundary {
    pub(crate) line: u32,
    pub(crate) side: BoundarySide,
}
