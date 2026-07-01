//! Host-provided skill manifest descriptors and progressive disclosure.
//!
//! Hosts parse files such as `SKILL.md`, databases, bundles, or other stores and
//! pass typed fields into this module. The reusable adapter crate validates data
//! only; it does not discover directories, follow symlinks, read home folders, or
//! load full skill bodies by ambient path.

#[cfg(test)]
mod tests;

type Text = alloc::string::String;
type Collection<T> = alloc::vec::Vec<T>;

const NAME_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("skill_name");
const DESCRIPTION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("skill_description");
const VERSION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("skill_version");
const CATEGORY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("skill_category");
const TAG_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("skill_tag");
const RELATED_SKILL_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("related_skill");
const PROVENANCE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("skill_provenance");
const TRUST_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("skill_trust_status");
const CONTENT_IDENTITY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("skill_content_identity");
const BODY_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("skill_body");

const TRUSTED_LABEL: &str = "trusted";
const UNTRUSTED_LABEL: &str = "untrusted";
const VERSION_SEPARATOR: char = '.';
const NAME_SEPARATOR_DASH: char = '-';
const NAME_SEPARATOR_UNDERSCORE: char = '_';
const SEMVER_SEGMENT_COUNT: usize = 3;
const MAX_ITEMS: usize = 32;
const MAX_DESCRIPTION_CHARS: usize = 512;

const _: () = assert!(SEMVER_SEGMENT_COUNT > 0);
const _: () = assert!(MAX_ITEMS > 0);
const _: () = assert!(MAX_DESCRIPTION_CHARS >= crate::validation::MIN_ADAPTER_FIELD_CHARS);

/// Descriptor-side effects that skill manifest validation explicitly does not perform.
pub const PROHIBITED_SKILL_MANIFEST_EFFECTS: &[&str] = &[
    "home-directory scan",
    "project-directory scan",
    "symlink resolution",
    "runtime-cache read",
    "skill body file read",
];

/// Character length of a lowercase BLAKE3 skill content identity.
pub const SKILL_CONTENT_HASH_HEX_CHARS: usize = crate::artifact::BLAKE3_HASH_HEX_CHARS;

/// Borrowed host-supplied skill manifest fields before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SkillManifestDraft<'a> {
    /// Stable user-facing skill name.
    pub name: Option<&'a str>,
    /// Safe human-facing summary shown before full skill loading.
    pub description: Option<&'a str>,
    /// Manifest contract version in `major.minor.patch` form.
    pub version: Option<&'a str>,
    /// Host-provided category metadata; it is not an authorization class.
    pub category: Option<&'a str>,
    /// Optional host-provided tag labels.
    pub tags: &'a [&'a str],
    /// Optional related skill names declared by this descriptor.
    pub related_skills: &'a [&'a str],
    /// Safe provenance label suitable for model-visible summaries.
    pub provenance: Option<&'a str>,
    /// Host trust status for the manifest source.
    pub trust_status: Option<&'a str>,
    /// Lowercase BLAKE3 identity for the full skill body supplied by the host.
    pub content_identity: Option<&'a str>,
}

/// Validated skill manifest descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkillManifest {
    name: Text,
    description: Text,
    version: Text,
    category: Text,
    tags: Collection<Text>,
    related_skills: Collection<Text>,
    provenance: Text,
    trust_status: SkillManifestTrustStatus,
    content_identity: crate::artifact::Hash,
}

impl SkillManifest {
    /// Validates host-supplied skill manifest fields as typed data only.
    ///
    /// # Errors
    ///
    /// Returns [`SkillManifestError`] when a required field is missing,
    /// malformed, unsupported, duplicated, too large, or not a safe BLAKE3
    /// content identity.
    pub fn validate(draft: &SkillManifestDraft<'_>) -> Result<Self, SkillManifestError> {
        let name = required_name(NAME_FIELD, draft.name)?;
        let description = required_description(draft.description)?;
        let version = required_version(draft.version)?;
        let category = required_name(CATEGORY_FIELD, draft.category)?;
        let tags = collect_unique_names(TAG_FIELD, draft.tags, DuplicatePolicy::RejectTags)?;
        let related_skills = collect_unique_names(
            RELATED_SKILL_FIELD,
            draft.related_skills,
            DuplicatePolicy::Dedup,
        )?;
        let provenance = required_text(PROVENANCE_FIELD, draft.provenance)?;
        let trust_status = SkillManifestTrustStatus::parse(draft.trust_status)?;
        let content_identity = required_content_identity(draft.content_identity)?;

        Ok(Self {
            name,
            description,
            version,
            category,
            tags,
            related_skills,
            provenance,
            trust_status,
            content_identity,
        })
    }

    /// Returns the validated skill name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the safe human-facing summary.
    #[must_use]
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Returns the manifest contract version.
    #[must_use]
    pub fn version(&self) -> &str {
        self.version.as_str()
    }

    /// Returns the host-provided category metadata.
    #[must_use]
    pub fn category(&self) -> &str {
        self.category.as_str()
    }

    /// Returns sorted unique tag labels.
    #[must_use]
    pub fn tags(&self) -> &[Text] {
        self.tags.as_slice()
    }

    /// Returns sorted unique related skill names.
    #[must_use]
    pub fn related_skills(&self) -> &[Text] {
        self.related_skills.as_slice()
    }

    /// Returns the safe provenance label.
    #[must_use]
    pub fn provenance(&self) -> &str {
        self.provenance.as_str()
    }

    /// Returns the host trust status.
    #[must_use]
    pub const fn trust_status(&self) -> SkillManifestTrustStatus {
        self.trust_status
    }

    /// Returns the safe content identity for the full body.
    #[must_use]
    pub fn content_identity(&self) -> &crate::artifact::Hash {
        &self.content_identity
    }

    fn resource_summary(&self) -> SkillResourceSummary {
        SkillResourceSummary {
            name: self.name.clone(),
            description: self.description.clone(),
            category: self.category.clone(),
            tags: self.tags.clone(),
            provenance: self.provenance.clone(),
        }
    }
}

/// Host trust status for a skill manifest source.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SkillManifestTrustStatus {
    /// Host has marked the skill source trusted for explicit full-body loading.
    Trusted,
    /// Host has not admitted the skill source for full-body loading.
    Untrusted,
}

impl SkillManifestTrustStatus {
    /// Returns the stable manifest label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => TRUSTED_LABEL,
            Self::Untrusted => UNTRUSTED_LABEL,
        }
    }

    fn parse(value: Option<&str>) -> Result<Self, SkillManifestError> {
        let normalized = required_text(TRUST_FIELD, value)?;
        match normalized.as_str() {
            TRUSTED_LABEL => Ok(Self::Trusted),
            UNTRUSTED_LABEL => Ok(Self::Untrusted),
            unknown => Err(SkillManifestError::UnsupportedValue {
                field: TRUST_FIELD.as_str(),
                value: Text::from(unknown),
            }),
        }
    }
}

/// Skill manifest validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SkillManifestError {
    /// A required manifest field was absent.
    MissingField {
        /// Missing field name.
        field: &'static str,
    },
    /// A field failed adapter-boundary validation.
    Contract(crate::AdapterContractError),
    /// A name-like field used an unsupported character or separator shape.
    MalformedName {
        /// Field containing the malformed name.
        field: &'static str,
        /// Malformed normalized value.
        value: Text,
    },
    /// A description exceeded the safe summary bound.
    DescriptionTooLong {
        /// Maximum accepted description characters.
        max_chars: usize,
        /// Actual description characters.
        actual_chars: usize,
    },
    /// Version did not use the supported semantic shape.
    UnsupportedVersion {
        /// Unsupported normalized version value.
        value: Text,
    },
    /// A tag label was duplicated after normalization.
    DuplicateTag {
        /// Duplicated tag value.
        tag: Text,
    },
    /// A collection exceeded the bounded item limit.
    TooManyItems {
        /// Field containing too many items.
        field: &'static str,
        /// Maximum supported items.
        max_items: usize,
        /// Actual item count.
        actual_items: usize,
    },
    /// Content identity failed lowercase BLAKE3 validation.
    InvalidContentIdentity {
        /// Field containing the unsafe identity.
        field: &'static str,
        /// Human-readable validation reason.
        reason: &'static str,
    },
    /// A manifest label used an unsupported value.
    UnsupportedValue {
        /// Field containing the unsupported value.
        field: &'static str,
        /// Unsupported normalized value.
        value: Text,
    },
}

impl core::fmt::Display for SkillManifestError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingField { field } => write!(formatter, "{field} is required"),
            Self::Contract(error) => write!(formatter, "{error}"),
            Self::MalformedName { field, value } => {
                write!(formatter, "{field} has malformed skill name {value}")
            }
            Self::DescriptionTooLong {
                max_chars,
                actual_chars,
            } => write!(
                formatter,
                "skill description supports at most {max_chars} characters; got {actual_chars}"
            ),
            Self::UnsupportedVersion { value } => {
                write!(formatter, "skill_version has unsupported version {value}")
            }
            Self::DuplicateTag { tag } => write!(formatter, "skill tag {tag} is duplicated"),
            Self::TooManyItems {
                field,
                max_items,
                actual_items,
            } => write!(
                formatter,
                "{field} supports at most {max_items} items; got {actual_items}"
            ),
            Self::InvalidContentIdentity { field, reason } => {
                write!(formatter, "{field} is invalid: {reason}")
            }
            Self::UnsupportedValue { field, value } => {
                write!(formatter, "{field} has unsupported skill value {value}")
            }
        }
    }
}

impl From<crate::AdapterContractError> for SkillManifestError {
    fn from(error: crate::AdapterContractError) -> Self {
        Self::Contract(error)
    }
}

/// Model-visible skill summary that omits full body content.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkillResourceSummary {
    name: Text,
    description: Text,
    category: Text,
    tags: Collection<Text>,
    provenance: Text,
}

impl SkillResourceSummary {
    /// Returns the skill name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the safe human-facing summary.
    #[must_use]
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Returns the host-provided category metadata.
    #[must_use]
    pub fn category(&self) -> &str {
        self.category.as_str()
    }

    /// Returns sorted tag labels.
    #[must_use]
    pub fn tags(&self) -> &[Text] {
        self.tags.as_slice()
    }

    /// Returns the safe provenance label.
    #[must_use]
    pub fn provenance(&self) -> &str {
        self.provenance.as_str()
    }
}

/// Skill resource advertisement catalog.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkillResourceCatalog {
    summaries: Collection<SkillResourceSummary>,
    diagnostics: Collection<SkillResourceDiagnostic>,
}

impl SkillResourceCatalog {
    /// Returns deterministic skill summaries admitted for advertisement.
    #[must_use]
    pub fn summaries(&self) -> &[SkillResourceSummary] {
        self.summaries.as_slice()
    }

    /// Returns deterministic catalog diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[SkillResourceDiagnostic] {
        self.diagnostics.as_slice()
    }
}

/// Skill catalog issue class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SkillResourceIssue {
    /// More than one manifest used the same skill name.
    DuplicateSkillName,
    /// A related skill name did not resolve to a supplied descriptor.
    UnresolvedRelatedSkill,
}

/// Safe diagnostic emitted while building skill resource summaries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkillResourceDiagnostic {
    issue: SkillResourceIssue,
    name: Text,
    related_skill: Option<Text>,
}

impl SkillResourceDiagnostic {
    /// Returns the diagnostic issue class.
    #[must_use]
    pub const fn issue(&self) -> SkillResourceIssue {
        self.issue
    }

    /// Returns the skill name associated with the issue.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the unresolved related skill, when present.
    #[must_use]
    pub fn related_skill(&self) -> Option<&str> {
        self.related_skill.as_deref()
    }
}

/// Borrowed full-body content explicitly supplied by a host load decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SkillBodyDraft<'a> {
    /// Skill name selected by the host.
    pub name: &'a str,
    /// Lowercase BLAKE3 identity for this body.
    pub content_identity: &'a str,
    /// Full body content supplied after explicit host approval.
    pub body: &'a str,
}

/// Loaded skill body selected for prompt construction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkillLoadedBody {
    name: Text,
    content_identity: crate::artifact::Hash,
    body: Text,
}

impl SkillLoadedBody {
    /// Returns the loaded skill name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the body content identity.
    #[must_use]
    pub fn content_identity(&self) -> &crate::artifact::Hash {
        &self.content_identity
    }

    /// Returns the full body content selected by the host.
    #[must_use]
    pub fn body(&self) -> &str {
        self.body.as_str()
    }
}

/// Inputs for skill progressive disclosure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SkillDisclosureRequest<'a> {
    /// Validated skill manifests supplied by the host.
    pub manifests: &'a [SkillManifest],
    /// Full bodies made available by an explicit host load action.
    pub loaded_bodies: &'a [SkillBodyDraft<'a>],
    /// Optional selected skill name to load into prompt context.
    pub selected_name: Option<&'a str>,
}

/// Progressive skill disclosure context for prompt construction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkillDisclosureContext {
    catalog: SkillResourceCatalog,
    selected_body: Option<SkillLoadedBody>,
}

impl SkillDisclosureContext {
    /// Returns resource summaries plus safe diagnostics.
    #[must_use]
    pub const fn catalog(&self) -> &SkillResourceCatalog {
        &self.catalog
    }

    /// Returns the selected full body, when a trusted matching body was loaded.
    #[must_use]
    pub const fn selected_body(&self) -> Option<&SkillLoadedBody> {
        self.selected_body.as_ref()
    }
}

/// Builds deterministic resource summaries from already validated manifests.
#[must_use]
pub fn advertise_skill_resources(manifests: &[SkillManifest]) -> SkillResourceCatalog {
    let mut summaries = Collection::new();
    let mut diagnostics = Collection::new();

    for manifest in manifests {
        push_related_diagnostics(manifest, manifests, &mut diagnostics);
        if duplicate_name(manifest.name(), manifests) {
            push_duplicate_diagnostic(manifest, &mut diagnostics);
            continue;
        }
        summaries.push(manifest.resource_summary());
    }

    summaries.sort_by(|left, right| left.name.cmp(&right.name));
    diagnostics.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then(left.related_skill.cmp(&right.related_skill))
    });
    SkillResourceCatalog {
        summaries,
        diagnostics,
    }
}

/// Builds progressive disclosure context without loading unselected bodies.
#[must_use]
pub fn build_skill_disclosure_context(
    request: SkillDisclosureRequest<'_>,
) -> SkillDisclosureContext {
    let catalog = advertise_skill_resources(request.manifests);
    let selected_body = request.selected_name.and_then(|selected_name| {
        select_loaded_body(request.manifests, request.loaded_bodies, selected_name)
    });

    SkillDisclosureContext {
        catalog,
        selected_body,
    }
}

fn select_loaded_body(
    manifests: &[SkillManifest],
    bodies: &[SkillBodyDraft<'_>],
    selected_name: &str,
) -> Option<SkillLoadedBody> {
    let manifest = manifests.iter().find(|item| item.name() == selected_name)?;
    if manifest.trust_status() != SkillManifestTrustStatus::Trusted {
        return None;
    }
    if duplicate_name(manifest.name(), manifests) {
        return None;
    }
    let body = bodies.iter().find(|candidate| {
        candidate.name == manifest.name()
            && candidate.content_identity == manifest.content_identity().as_str()
    })?;
    let validated_body = required_text(BODY_FIELD, Some(body.body)).ok()?;
    Some(SkillLoadedBody {
        name: manifest.name.clone(),
        content_identity: manifest.content_identity.clone(),
        body: validated_body,
    })
}

fn push_related_diagnostics(
    manifest: &SkillManifest,
    manifests: &[SkillManifest],
    diagnostics: &mut Collection<SkillResourceDiagnostic>,
) {
    for related_skill in manifest.related_skills() {
        if !contains_manifest_name(manifests, related_skill.as_str()) {
            diagnostics.push(SkillResourceDiagnostic {
                issue: SkillResourceIssue::UnresolvedRelatedSkill,
                name: Text::from(manifest.name()),
                related_skill: Some(related_skill.clone()),
            });
        }
    }
}

fn push_duplicate_diagnostic(
    manifest: &SkillManifest,
    diagnostics: &mut Collection<SkillResourceDiagnostic>,
) {
    if diagnostics.iter().any(|diagnostic| {
        diagnostic.issue == SkillResourceIssue::DuplicateSkillName
            && diagnostic.name == manifest.name()
            && diagnostic.related_skill.is_none()
    }) {
        return;
    }
    diagnostics.push(SkillResourceDiagnostic {
        issue: SkillResourceIssue::DuplicateSkillName,
        name: Text::from(manifest.name()),
        related_skill: None,
    });
}

fn duplicate_name(name: &str, manifests: &[SkillManifest]) -> bool {
    manifests
        .iter()
        .filter(|manifest| manifest.name() == name)
        .count()
        > 1
}

fn contains_manifest_name(manifests: &[SkillManifest], name: &str) -> bool {
    manifests.iter().any(|manifest| manifest.name() == name)
}

fn required_description(value: Option<&str>) -> Result<Text, SkillManifestError> {
    let description = required_text(DESCRIPTION_FIELD, value)?;
    let actual_chars = description.chars().count();
    if actual_chars > MAX_DESCRIPTION_CHARS {
        return Err(SkillManifestError::DescriptionTooLong {
            max_chars: MAX_DESCRIPTION_CHARS,
            actual_chars,
        });
    }
    Ok(description)
}

fn required_version(value: Option<&str>) -> Result<Text, SkillManifestError> {
    let version = required_text(VERSION_FIELD, value)?;
    if !valid_version(version.as_str()) {
        return Err(SkillManifestError::UnsupportedVersion { value: version });
    }
    Ok(version)
}

fn required_name(
    field: crate::validation::FieldName,
    value: Option<&str>,
) -> Result<Text, SkillManifestError> {
    let name = required_text(field, value)?;
    validate_name(field, name.as_str())?;
    Ok(name)
}

fn required_text(
    field: crate::validation::FieldName,
    value: Option<&str>,
) -> Result<Text, SkillManifestError> {
    match value {
        Some(raw) => crate::validation::normalized_adapter_field(field, raw).map_err(Into::into),
        None => Err(SkillManifestError::MissingField {
            field: field.as_str(),
        }),
    }
}

fn required_content_identity(
    value: Option<&str>,
) -> Result<crate::artifact::Hash, SkillManifestError> {
    let identity = required_text(CONTENT_IDENTITY_FIELD, value)?;
    crate::artifact::Hash::new(identity.as_str()).map_err(|error| content_identity_error(&error))
}

fn content_identity_error(error: &crate::artifact::Error) -> SkillManifestError {
    match error {
        crate::artifact::Error::InvalidHash { reason, .. } => {
            SkillManifestError::InvalidContentIdentity {
                field: CONTENT_IDENTITY_FIELD.as_str(),
                reason,
            }
        }
        crate::artifact::Error::PayloadTooLong { .. }
        | crate::artifact::Error::PayloadContainsNul { .. } => {
            SkillManifestError::InvalidContentIdentity {
                field: CONTENT_IDENTITY_FIELD.as_str(),
                reason: "skill content identity must be lowercase BLAKE3 hex",
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DuplicatePolicy {
    RejectTags,
    Dedup,
}

fn collect_unique_names(
    field: crate::validation::FieldName,
    values: &[&str],
    duplicate_policy: DuplicatePolicy,
) -> Result<Collection<Text>, SkillManifestError> {
    validate_item_count(field, values.len())?;
    let mut names = Collection::with_capacity(values.len());
    for value in values {
        let name = required_text(field, Some(*value))?;
        validate_name(field, name.as_str())?;
        if names.iter().any(|item| item == &name) {
            if duplicate_policy == DuplicatePolicy::RejectTags {
                return Err(SkillManifestError::DuplicateTag { tag: name });
            }
            continue;
        }
        names.push(name);
    }
    names.sort();
    Ok(names)
}

fn validate_item_count(
    field: crate::validation::FieldName,
    item_count: usize,
) -> Result<(), SkillManifestError> {
    if item_count > MAX_ITEMS {
        return Err(SkillManifestError::TooManyItems {
            field: field.as_str(),
            max_items: MAX_ITEMS,
            actual_items: item_count,
        });
    }
    Ok(())
}

fn validate_name(
    field: crate::validation::FieldName,
    value: &str,
) -> Result<(), SkillManifestError> {
    if valid_name(value) {
        Ok(())
    } else {
        Err(SkillManifestError::MalformedName {
            field: field.as_str(),
            value: Text::from(value),
        })
    }
}

fn valid_name(value: &str) -> bool {
    let mut previous_separator = false;
    let mut saw_character = false;
    for character in value.chars() {
        let separator = character == NAME_SEPARATOR_DASH || character == NAME_SEPARATOR_UNDERSCORE;
        if separator {
            if !saw_character || previous_separator {
                return false;
            }
            previous_separator = true;
            continue;
        }
        if !(character.is_ascii_lowercase() || character.is_ascii_digit()) {
            return false;
        }
        previous_separator = false;
        saw_character = true;
    }
    saw_character && !previous_separator
}

fn valid_version(value: &str) -> bool {
    let mut segments_seen: usize = 0;
    for segment in value.split(VERSION_SEPARATOR) {
        segments_seen = segments_seen.saturating_add(1);
        if segment.is_empty() || !segment.chars().all(|character| character.is_ascii_digit()) {
            return false;
        }
    }
    segments_seen == SEMVER_SEGMENT_COUNT
}
