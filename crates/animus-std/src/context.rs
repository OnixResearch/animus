//! Standard-library context resolver shells.
//!
//! These resolvers own local filesystem authority for embedders that opt into
//! them. Reusable crates continue to receive only already-loaded candidates
//! through `animus-adapters` contracts.

const BYTES_PER_KIBIBYTE: u32 = 1_024;
const DEFAULT_MAX_CONTEXT_KIBIBYTES: u32 = 64;
const DEFAULT_MAX_CONTEXT_BYTES: u32 =
    match DEFAULT_MAX_CONTEXT_KIBIBYTES.checked_mul(BYTES_PER_KIBIBYTE) {
        Some(bytes) => bytes,
        None => u32::MAX,
    };
const DEFAULT_MAX_DIRECTORY_ENTRIES: u32 = 128;
const MIN_CONTEXT_BYTES: u32 = 1;
const MIN_DIRECTORY_ENTRIES: u32 = 1;
const DIRECTORY_SUFFIX: char = '/';
const DIFF_TARGET_LABEL: &str = "working";
const DIFF_KIND_LABEL: &str = "diff";
const BINARY_NUL_BYTE: u8 = 0;
const LINE_SEPARATOR: char = '\n';

/// Maximum local context body bytes admitted by the default std policy.
pub const STD_CONTEXT_DEFAULT_MAX_BYTES: u32 = DEFAULT_MAX_CONTEXT_BYTES;

/// Maximum local directory entries admitted by the default std policy.
pub const STD_CONTEXT_DEFAULT_MAX_DIRECTORY_ENTRIES: u32 = DEFAULT_MAX_DIRECTORY_ENTRIES;

/// Policy for explicit std-shell local context resolvers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Policy {
    root: std::path::PathBuf,
    max_body_bytes: u32,
    max_directory_entries: u32,
    allow_files: bool,
    allow_directories: bool,
    allow_diffs: bool,
}

impl Policy {
    /// Builds a policy allowing local file, directory, and preloaded diff roles under a root.
    #[must_use]
    pub fn allow_local(root: impl Into<std::path::PathBuf>) -> Self {
        Self {
            root: root.into(),
            max_body_bytes: DEFAULT_MAX_CONTEXT_BYTES,
            max_directory_entries: DEFAULT_MAX_DIRECTORY_ENTRIES,
            allow_files: true,
            allow_directories: true,
            allow_diffs: true,
        }
    }

    /// Builds a policy that denies every local context role under a root.
    #[must_use]
    pub fn deny_all(root: impl Into<std::path::PathBuf>) -> Self {
        Self {
            root: root.into(),
            max_body_bytes: DEFAULT_MAX_CONTEXT_BYTES,
            max_directory_entries: DEFAULT_MAX_DIRECTORY_ENTRIES,
            allow_files: false,
            allow_directories: false,
            allow_diffs: false,
        }
    }

    /// Returns a copy with a custom maximum body byte bound.
    #[must_use]
    pub fn with_max_body_bytes(mut self, max_body_bytes: u32) -> Self {
        self.max_body_bytes = max_body_bytes;
        self
    }

    /// Returns a copy with a custom maximum directory entry bound.
    #[must_use]
    pub fn with_max_directory_entries(mut self, max_directory_entries: u32) -> Self {
        self.max_directory_entries = max_directory_entries;
        self
    }

    /// Returns a copy with file role admission changed.
    #[must_use]
    pub fn with_files_allowed(mut self, allowed: bool) -> Self {
        self.allow_files = allowed;
        self
    }

    /// Returns a copy with directory role admission changed.
    #[must_use]
    pub fn with_directories_allowed(mut self, allowed: bool) -> Self {
        self.allow_directories = allowed;
        self
    }

    /// Returns a copy with diff role admission changed.
    #[must_use]
    pub fn with_diffs_allowed(mut self, allowed: bool) -> Self {
        self.allow_diffs = allowed;
        self
    }

    /// Returns the local root used for relative target resolution.
    #[must_use]
    pub fn root(&self) -> &std::path::Path {
        self.root.as_path()
    }

    /// Returns the maximum admitted body bytes.
    #[must_use]
    pub const fn max_body_bytes(&self) -> u32 {
        self.max_body_bytes
    }

    /// Returns the maximum admitted directory entries.
    #[must_use]
    pub const fn max_directory_entries(&self) -> u32 {
        self.max_directory_entries
    }
}

/// Explicit no-op context resolver. Installing it grants no context authority.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NullResolver;

impl animus_adapters::FileContextResolver for NullResolver {
    fn resolve_file(
        &self,
        _reference: &animus_adapters::ContextReference,
    ) -> Result<animus_adapters::ResolvedContextCandidate, animus_adapters::ContextReferenceError>
    {
        Err(animus_adapters::ContextReferenceError::ResolverDenied)
    }
}

impl animus_adapters::DirectoryContextResolver for NullResolver {
    fn resolve_directory(
        &self,
        _reference: &animus_adapters::ContextReference,
    ) -> Result<animus_adapters::ResolvedContextCandidate, animus_adapters::ContextReferenceError>
    {
        Err(animus_adapters::ContextReferenceError::ResolverDenied)
    }
}

impl animus_adapters::DiffContextResolver for NullResolver {
    fn resolve_diff(
        &self,
        _reference: &animus_adapters::ContextReference,
    ) -> Result<animus_adapters::ResolvedContextCandidate, animus_adapters::ContextReferenceError>
    {
        Err(animus_adapters::ContextReferenceError::ResolverDenied)
    }
}

impl animus_adapters::ImageContextResolver for NullResolver {
    fn resolve_image(
        &self,
        _reference: &animus_adapters::ContextReference,
    ) -> Result<animus_adapters::ResolvedContextCandidate, animus_adapters::ContextReferenceError>
    {
        Err(animus_adapters::ContextReferenceError::ResolverDenied)
    }
}

impl animus_adapters::UrlContextResolver for NullResolver {
    fn resolve_url(
        &self,
        _reference: &animus_adapters::ContextReference,
    ) -> Result<animus_adapters::ResolvedContextCandidate, animus_adapters::ContextReferenceError>
    {
        Err(animus_adapters::ContextReferenceError::ResolverDenied)
    }
}

/// Policy-gated local file resolver.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalFileResolver {
    policy: Policy,
}

impl LocalFileResolver {
    /// Builds a local file resolver from explicit policy.
    #[must_use]
    pub fn new(policy: Policy) -> Self {
        Self { policy }
    }

    /// Returns resolver policy.
    #[must_use]
    pub const fn policy(&self) -> &Policy {
        &self.policy
    }
}

impl animus_adapters::FileContextResolver for LocalFileResolver {
    fn resolve_file(
        &self,
        reference: &animus_adapters::ContextReference,
    ) -> Result<animus_adapters::ResolvedContextCandidate, animus_adapters::ContextReferenceError>
    {
        ensure_allowed(self.policy.allow_files)?;
        let path = resolve_under_root(&self.policy, reference.normalized_target())?;
        let bytes = read_bounded_file(path.as_path(), self.policy.max_body_bytes)?;
        let content = String::from_utf8(bytes)
            .map_err(|_error| animus_adapters::ContextReferenceError::ResolverMalformedOutput)?;
        Ok(animus_adapters::ResolvedContextCandidate::new(
            reference.clone(),
            animus_adapters::ContextResolverKind::File,
            content,
        ))
    }
}

/// Policy-gated local directory listing resolver.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalDirectoryResolver {
    policy: Policy,
}

impl LocalDirectoryResolver {
    /// Builds a local directory resolver from explicit policy.
    #[must_use]
    pub fn new(policy: Policy) -> Self {
        Self { policy }
    }

    /// Returns resolver policy.
    #[must_use]
    pub const fn policy(&self) -> &Policy {
        &self.policy
    }
}

impl animus_adapters::DirectoryContextResolver for LocalDirectoryResolver {
    fn resolve_directory(
        &self,
        reference: &animus_adapters::ContextReference,
    ) -> Result<animus_adapters::ResolvedContextCandidate, animus_adapters::ContextReferenceError>
    {
        ensure_allowed(self.policy.allow_directories)?;
        let path = resolve_under_root(&self.policy, reference.normalized_target())?;
        let content = read_directory_listing(path.as_path(), &self.policy)?;
        Ok(animus_adapters::ResolvedContextCandidate::new(
            reference.clone(),
            animus_adapters::ContextResolverKind::Directory,
            content,
        ))
    }
}

/// Policy-gated preloaded diff resolver.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalDiffResolver {
    policy: Policy,
    diff_text: String,
}

impl LocalDiffResolver {
    /// Builds a diff resolver from explicit policy and already-loaded diff text.
    #[must_use]
    pub fn new(policy: Policy, diff_text: impl Into<String>) -> Self {
        Self {
            policy,
            diff_text: diff_text.into(),
        }
    }

    /// Returns resolver policy.
    #[must_use]
    pub const fn policy(&self) -> &Policy {
        &self.policy
    }

    /// Returns the already-loaded diff text.
    #[must_use]
    pub fn diff_text(&self) -> &str {
        self.diff_text.as_str()
    }
}

impl animus_adapters::DiffContextResolver for LocalDiffResolver {
    fn resolve_diff(
        &self,
        reference: &animus_adapters::ContextReference,
    ) -> Result<animus_adapters::ResolvedContextCandidate, animus_adapters::ContextReferenceError>
    {
        ensure_allowed(self.policy.allow_diffs)?;
        ensure_diff_target(reference.normalized_target())?;
        ensure_text_bounds(self.diff_text.as_bytes(), self.policy.max_body_bytes)?;
        reject_binary_body(self.diff_text.as_bytes())?;
        Ok(animus_adapters::ResolvedContextCandidate::new(
            reference.clone(),
            animus_adapters::ContextResolverKind::Diff,
            self.diff_text.clone(),
        ))
    }
}

fn ensure_allowed(allowed: bool) -> Result<(), animus_adapters::ContextReferenceError> {
    if allowed {
        return Ok(());
    }
    Err(animus_adapters::ContextReferenceError::ResolverDenied)
}

fn u32_to_usize_count(value: u32) -> Result<usize, animus_adapters::ContextReferenceError> {
    usize::try_from(value).map_err(|_error| animus_adapters::ContextReferenceError::InvalidBudget)
}

fn resolve_under_root(
    policy: &Policy,
    normalized_target: &str,
) -> Result<std::path::PathBuf, animus_adapters::ContextReferenceError> {
    ensure_policy_bounds(policy)?;
    ensure_relative_target(normalized_target)?;
    let root = policy
        .root
        .canonicalize()
        .map_err(|_error| animus_adapters::ContextReferenceError::ResolverUnavailable)?;
    let joined = root.join(normalized_target);
    let resolved = joined
        .canonicalize()
        .map_err(|_error| animus_adapters::ContextReferenceError::ResolverUnavailable)?;
    if !resolved.starts_with(root.as_path()) {
        return Err(animus_adapters::ContextReferenceError::PathEscape {
            reason: "std context target escaped root",
        });
    }
    Ok(resolved)
}

fn ensure_policy_bounds(policy: &Policy) -> Result<(), animus_adapters::ContextReferenceError> {
    if policy.max_body_bytes < MIN_CONTEXT_BYTES
        || policy.max_directory_entries < MIN_DIRECTORY_ENTRIES
    {
        return Err(animus_adapters::ContextReferenceError::InvalidBudget);
    }
    Ok(())
}

fn ensure_relative_target(target: &str) -> Result<(), animus_adapters::ContextReferenceError> {
    let path = std::path::Path::new(target);
    if path.is_absolute() || path.components().any(is_escape_component) {
        return Err(animus_adapters::ContextReferenceError::PathEscape {
            reason: "std context target must stay under root",
        });
    }
    Ok(())
}

fn is_escape_component(component: std::path::Component<'_>) -> bool {
    matches!(
        component,
        std::path::Component::ParentDir
            | std::path::Component::RootDir
            | std::path::Component::Prefix(_)
    )
}

fn read_bounded_file(
    path: &std::path::Path,
    max_bytes: u32,
) -> Result<Vec<u8>, animus_adapters::ContextReferenceError> {
    let bytes = std::fs::read(path)
        .map_err(|_error| animus_adapters::ContextReferenceError::ResolverUnavailable)?;
    ensure_text_bounds(bytes.as_slice(), max_bytes)?;
    reject_binary_body(bytes.as_slice())?;
    Ok(bytes)
}

fn ensure_text_bounds(
    bytes: &[u8],
    max_bytes: u32,
) -> Result<(), animus_adapters::ContextReferenceError> {
    if bytes.len() > u32_to_usize_count(max_bytes)? {
        return Err(animus_adapters::ContextReferenceError::ResolverOversized);
    }
    Ok(())
}

fn reject_binary_body(bytes: &[u8]) -> Result<(), animus_adapters::ContextReferenceError> {
    if bytes.contains(&BINARY_NUL_BYTE) {
        return Err(animus_adapters::ContextReferenceError::ResolverMalformedOutput);
    }
    Ok(())
}

fn read_directory_listing(
    path: &std::path::Path,
    policy: &Policy,
) -> Result<String, animus_adapters::ContextReferenceError> {
    debug_assert!(policy.max_body_bytes >= MIN_CONTEXT_BYTES);
    debug_assert!(policy.max_directory_entries >= MIN_DIRECTORY_ENTRIES);
    let mut entries = std::fs::read_dir(path)
        .map_err(|_error| animus_adapters::ContextReferenceError::ResolverUnavailable)?
        .map(|entry| {
            entry.map_err(|_error| animus_adapters::ContextReferenceError::ResolverUnavailable)
        })
        .collect::<Result<Vec<_>, _>>()?;
    if entries.len() > u32_to_usize_count(policy.max_directory_entries)? {
        return Err(animus_adapters::ContextReferenceError::ResolverOversized);
    }
    entries.sort_by_key(std::fs::DirEntry::file_name);
    let mut listing = String::new();
    for entry in entries {
        let file_name = entry.file_name();
        let label = file_name
            .to_str()
            .ok_or(animus_adapters::ContextReferenceError::ResolverMalformedOutput)?;
        reject_unsafe_file_name(file_name.as_os_str())?;
        listing.push_str(label);
        if entry
            .file_type()
            .map_err(|_error| animus_adapters::ContextReferenceError::ResolverUnavailable)?
            .is_dir()
        {
            listing.push(DIRECTORY_SUFFIX);
        }
        listing.push(LINE_SEPARATOR);
    }
    ensure_text_bounds(listing.as_bytes(), policy.max_body_bytes)?;
    Ok(listing)
}

fn reject_unsafe_file_name(
    file_name: &std::ffi::OsStr,
) -> Result<(), animus_adapters::ContextReferenceError> {
    let path = std::path::Path::new(file_name);
    if path.components().any(is_escape_component) {
        return Err(animus_adapters::ContextReferenceError::PathEscape {
            reason: "std context directory entry must stay relative",
        });
    }
    Ok(())
}

fn ensure_diff_target(target: &str) -> Result<(), animus_adapters::ContextReferenceError> {
    if target == DIFF_TARGET_LABEL || target.starts_with(DIFF_KIND_LABEL) {
        return Ok(());
    }
    Err(animus_adapters::ContextReferenceError::UnsafeTarget {
        reason: "std diff context target is not admitted",
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_adapters::{
        DiffContextResolver, DirectoryContextResolver, FileContextResolver, ImageContextResolver,
        UrlContextResolver,
    };

    const TEST_ROOT_DIR: &str = "animus-std-context-resolver-tests";
    const POSITIVE_TEST_ROOT: &str = "positive";
    const NEGATIVE_TEST_ROOT: &str = "negative";
    const MISSING_TEST_ROOT: &str = "missing";
    const SRC_DIR: &str = "src";
    const NESTED_DIR: &str = "nested";
    const LIB_RS: &str = "lib.rs";
    const BINARY_FILE: &str = "binary.bin";
    const LARGE_FILE: &str = "large.txt";
    const FILE_REFERENCE: &str = "@src/lib.rs";
    const DIRECTORY_REFERENCE: &str = "@src/";
    const DIFF_REFERENCE: &str = "@diff:working";
    const IMAGE_REFERENCE: &str = "@image:assets/logo.png";
    const URL_REFERENCE: &str = "@https://example.test/context";
    const MISSING_REFERENCE: &str = "@src/missing.txt";
    const BINARY_REFERENCE: &str = "@src/binary.bin";
    const LARGE_REFERENCE: &str = "@src/large.txt";
    const FILE_BODY: &str = "alpha\nbeta\n";
    const DIFF_BODY: &str = "diff --git a/src/lib.rs b/src/lib.rs\n";
    const BINARY_BODY: &[u8] = b"alpha\0beta";
    const LARGE_BODY: &str = "alpha beta gamma";
    const SMALL_BODY_BYTE_LIMIT: u32 = 4;
    const VISIBLE_BYTES: u32 = 64;
    const VISIBLE_LINES: u32 = 8;

    #[test]
    fn local_resolvers_expand_and_budget() -> Result<(), animus_adapters::ContextReferenceError> {
        let root = fixture_root(POSITIVE_TEST_ROOT);
        write_fixture_files(root.as_path());
        let policy = Policy::allow_local(root.as_path());
        let file = LocalFileResolver::new(policy.clone());
        let directory = LocalDirectoryResolver::new(policy.clone());
        let diff = LocalDiffResolver::new(policy, DIFF_BODY);
        let resolvers = animus_adapters::ContextResolverSet::empty()
            .with_file(&file)
            .with_directory(&directory)
            .with_diff(&diff);

        let reference = animus_adapters::parse_context_reference(FILE_REFERENCE)?;
        let candidate = animus_adapters::expand_context_reference(
            &reference,
            animus_adapters::ContextReferencePolicy::allow_local(),
            &resolvers,
        )?;
        let fragment = animus_adapters::apply_context_budget(
            candidate,
            animus_adapters::ContextReferenceBudget::new(
                VISIBLE_BYTES,
                VISIBLE_LINES,
                animus_adapters::ContextOversizedPolicy::Truncate,
            )?,
        )?;
        assert_eq!(
            fragment.status(),
            animus_adapters::ContextReferenceStatus::Included
        );
        assert_eq!(fragment.visible_text(), Some(FILE_BODY));

        let directory_reference = animus_adapters::parse_context_reference(DIRECTORY_REFERENCE)?;
        let listing = directory.resolve_directory(&directory_reference)?;
        assert!(listing.content().contains(LIB_RS));
        assert!(listing.content().contains("nested/"));

        let diff_reference = animus_adapters::parse_context_reference(DIFF_REFERENCE)?;
        let diff_candidate = diff.resolve_diff(&diff_reference)?;
        assert_eq!(diff_candidate.content(), DIFF_BODY);
        Ok(())
    }

    #[test]
    fn local_resolvers_fail_closed_for_denied_and_unsafe_targets()
    -> Result<(), animus_adapters::ContextReferenceError> {
        let root = fixture_root(NEGATIVE_TEST_ROOT);
        write_fixture_files(root.as_path());
        let denied_policy = Policy::deny_all(root.as_path());
        let denied_file = LocalFileResolver::new(denied_policy);
        let file_reference = animus_adapters::parse_context_reference(FILE_REFERENCE)?;
        assert_eq!(
            denied_file.resolve_file(&file_reference),
            Err(animus_adapters::ContextReferenceError::ResolverDenied)
        );

        let policy = Policy::allow_local(root.as_path()).with_max_body_bytes(SMALL_BODY_BYTE_LIMIT);
        let large_body_resolver = LocalFileResolver::new(policy.clone());
        let large_reference = animus_adapters::parse_context_reference(LARGE_REFERENCE)?;
        assert_eq!(
            large_body_resolver.resolve_file(&large_reference),
            Err(animus_adapters::ContextReferenceError::ResolverOversized)
        );

        let binary_reference = animus_adapters::parse_context_reference(BINARY_REFERENCE)?;
        let binary_file = LocalFileResolver::new(Policy::allow_local(root));
        assert_eq!(
            binary_file.resolve_file(&binary_reference),
            Err(animus_adapters::ContextReferenceError::ResolverMalformedOutput)
        );

        let null = NullResolver;
        let image = animus_adapters::parse_context_reference(IMAGE_REFERENCE)?;
        let url = animus_adapters::parse_context_reference(URL_REFERENCE)?;
        assert_eq!(
            null.resolve_image(&image),
            Err(animus_adapters::ContextReferenceError::ResolverDenied)
        );
        assert_eq!(
            null.resolve_url(&url),
            Err(animus_adapters::ContextReferenceError::ResolverDenied)
        );
        Ok(())
    }

    #[test]
    fn local_resolvers_report_missing_files_without_private_paths()
    -> Result<(), animus_adapters::ContextReferenceError> {
        let root = fixture_root(MISSING_TEST_ROOT);
        write_fixture_files(root.as_path());
        let file = LocalFileResolver::new(Policy::allow_local(root));
        let missing = animus_adapters::parse_context_reference(MISSING_REFERENCE)?;
        assert_eq!(
            file.resolve_file(&missing),
            Err(animus_adapters::ContextReferenceError::ResolverUnavailable)
        );
        Ok(())
    }

    fn fixture_root(suffix: &str) -> std::path::PathBuf {
        let root =
            std::env::temp_dir().join(format!("{TEST_ROOT_DIR}-{suffix}-{}", std::process::id()));
        let _ignored = std::fs::remove_dir_all(root.as_path());
        std::fs::create_dir_all(root.join(SRC_DIR).join(NESTED_DIR))
            .expect("fixture directory should be created");
        root
    }

    fn write_fixture_files(root: &std::path::Path) {
        let src = root.join(SRC_DIR);
        std::fs::write(src.join(LIB_RS), FILE_BODY).expect("text fixture should be written");
        std::fs::write(src.join(BINARY_FILE), BINARY_BODY)
            .expect("binary fixture should be written");
        std::fs::write(src.join(LARGE_FILE), LARGE_BODY).expect("large fixture should be written");
    }
}
