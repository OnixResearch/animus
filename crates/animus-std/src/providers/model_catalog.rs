//! Versioned shipped and user model catalog admission.
//!
//! The module is pure over caller-supplied JSON and values. File loading is a
//! separate shell helper. It never reads credentials or provider endpoints.

use serde::Deserialize;

/// Stable model catalog schema.
pub const MODEL_CATALOG_SCHEMA: &str = "animus.model-catalog.v1";

const MAX_CATALOG_CONNECTIONS: usize = 32;
const MAX_CATALOG_MODELS: usize = 128;
const CATALOG_HASH_DOMAIN: &str = "animus.model-catalog.v1";
const HASH_SEPARATOR: &str = "|";
const FIELD_SCHEMA: &str = "schema";
const FIELD_SOURCE: &str = "source";
const FIELD_CONNECTIONS: &str = "connections";
const FIELD_MODELS: &str = "models";
const FIELD_CREDENTIAL_ENV: &str = "credential_env";
const FIELD_CREDENTIAL_VALUE: &str = "credential_value";
const FIELD_SELECTION: &str = "selection";

/// Stable model catalog failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelCatalogIssue {
    /// The catalog schema is unsupported.
    UnsupportedSchema,
    /// A label or identifier is unsafe.
    InvalidIdentifier,
    /// A bounded collection is empty or too large.
    InvalidCollection,
    /// One source contains a duplicate identity.
    DuplicateIdentity,
    /// An entry references an absent connection.
    MissingConnection,
    /// A user source tried to declare a built-in connection.
    InvalidUserConnectionClass,
    /// A user source tried to redefine a shipped built-in connection.
    ReservedConnection,
    /// Catalog content contains literal credential material.
    LiteralCredential,
    /// The JSON shape or syntax is invalid.
    InvalidJson,
    /// The catalog file could not be read.
    ReadFailed,
    /// Automatic selection tried to use a custom connection.
    AutomaticCustomConnection,
    /// The selected stable model ID is unavailable.
    ModelUnavailable,
}

/// Safe bounded model catalog failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelCatalogError {
    issue: ModelCatalogIssue,
    source_label: String,
    field_class: &'static str,
}

impl ModelCatalogError {
    /// Returns the stable issue class.
    #[must_use]
    pub const fn issue(&self) -> ModelCatalogIssue {
        self.issue
    }

    /// Returns the safe catalog source label.
    #[must_use]
    pub fn source_label(&self) -> &str {
        self.source_label.as_str()
    }

    /// Returns the stable field class.
    #[must_use]
    pub const fn field_class(&self) -> &'static str {
        self.field_class
    }
}

impl core::fmt::Display for ModelCatalogError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "model catalog error: source={} field={} issue={:?}",
            self.source_label, self.field_class, self.issue
        )
    }
}

impl std::error::Error for ModelCatalogError {}

/// Declared connection class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelCatalogConnectionClass {
    /// Repository or product supplied connection.
    BuiltIn,
    /// Operator supplied connection that is manual-only.
    Custom,
}

impl ModelCatalogConnectionClass {
    const fn as_str(self) -> &'static str {
        match self {
            Self::BuiltIn => "built_in",
            Self::Custom => "custom",
        }
    }
}

/// Selection source for one catalog lookup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelCatalogSelectionMode {
    /// Exact operator-requested selection.
    Manual,
    /// Retry, routing, or billing fallback selection.
    Automatic,
}

/// Borrowed connection fields before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModelCatalogConnectionDraft<'a> {
    /// Stable connection ID.
    pub id: &'a str,
    /// Built-in or custom class.
    pub class: ModelCatalogConnectionClass,
    /// Optional environment-variable credential reference.
    pub credential_env: Option<&'a str>,
    /// Forbidden literal credential candidate.
    pub credential_value: Option<&'a str>,
}

/// Borrowed model entry fields before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModelCatalogEntryDraft<'a> {
    /// Stable user-facing model ID used for merge and selection.
    pub id: &'a str,
    /// Exact connection ID.
    pub connection_id: &'a str,
    /// Exact provider ID.
    pub provider_id: &'a str,
    /// Exact provider-native model ID.
    pub model_id: &'a str,
}

/// Borrowed catalog fields before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModelCatalogDraft<'a> {
    /// Exact catalog schema.
    pub schema: &'a str,
    /// Safe source label.
    pub source_label: &'a str,
    /// Connection rows.
    pub connections: &'a [ModelCatalogConnectionDraft<'a>],
    /// Model rows.
    pub models: &'a [ModelCatalogEntryDraft<'a>],
}

/// Validated model catalog connection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelCatalogConnection {
    id: super::ProviderId,
    class: ModelCatalogConnectionClass,
    credential_env: Option<String>,
}

impl ModelCatalogConnection {
    /// Returns the stable connection ID.
    #[must_use]
    pub const fn id(&self) -> &super::ProviderId {
        &self.id
    }

    /// Returns the connection class.
    #[must_use]
    pub const fn class(&self) -> ModelCatalogConnectionClass {
        self.class
    }

    /// Returns the optional environment-variable credential reference.
    #[must_use]
    pub fn credential_env(&self) -> Option<&str> {
        self.credential_env.as_deref()
    }
}

/// Validated model catalog entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelCatalogEntry {
    id: super::ModelId,
    connection_id: super::ProviderId,
    provider_id: super::ProviderId,
    model_id: super::ModelId,
}

impl ModelCatalogEntry {
    /// Returns the stable model ID.
    #[must_use]
    pub const fn id(&self) -> &super::ModelId {
        &self.id
    }

    /// Returns the exact connection ID.
    #[must_use]
    pub const fn connection_id(&self) -> &super::ProviderId {
        &self.connection_id
    }

    /// Returns the exact provider ID.
    #[must_use]
    pub const fn provider_id(&self) -> &super::ProviderId {
        &self.provider_id
    }

    /// Returns the provider-native model ID.
    #[must_use]
    pub const fn model_id(&self) -> &super::ModelId {
        &self.model_id
    }
}

/// Validated owned versioned model catalog.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelCatalog {
    source_label: String,
    connections: Vec<ModelCatalogConnection>,
    models: Vec<ModelCatalogEntry>,
    catalog_hash: String,
}

impl ModelCatalog {
    /// Validates one catalog source without reading files or credentials.
    ///
    /// # Errors
    ///
    /// Returns [`ModelCatalogError`] for malformed, duplicate, secret-bearing,
    /// missing-reference, or overbound input.
    ///
    /// # Panics
    ///
    /// Panics if validated non-empty collections or deterministic identity construction regress.
    // r[impl animus.catalog.version]
    // r[impl animus.catalog.secret]
    pub fn validate(draft: &ModelCatalogDraft<'_>) -> Result<Self, ModelCatalogError> {
        let source_label = validate_source_label(draft.source_label)?;
        if draft.schema != MODEL_CATALOG_SCHEMA {
            return Err(error(
                ModelCatalogIssue::UnsupportedSchema,
                source_label.as_str(),
                FIELD_SCHEMA,
            ));
        }
        validate_collection(
            draft.connections.len(),
            MAX_CATALOG_CONNECTIONS,
            source_label.as_str(),
            FIELD_CONNECTIONS,
        )?;
        validate_collection(
            draft.models.len(),
            MAX_CATALOG_MODELS,
            source_label.as_str(),
            FIELD_MODELS,
        )?;
        let connections = validate_connections(draft.connections, source_label.as_str())?;
        let models = validate_models(draft.models, connections.as_slice(), source_label.as_str())?;
        let catalog_hash = catalog_hash(connections.as_slice(), models.as_slice());
        let catalog = Self {
            source_label,
            connections,
            models,
            catalog_hash,
        };
        assert!(!catalog.connections().is_empty());
        assert!(!catalog.models().is_empty());
        assert!(!catalog.catalog_hash().is_empty());
        Ok(catalog)
    }

    /// Returns the safe source label.
    #[must_use]
    pub fn source_label(&self) -> &str {
        self.source_label.as_str()
    }

    /// Returns validated connection rows.
    #[must_use]
    pub fn connections(&self) -> &[ModelCatalogConnection] {
        self.connections.as_slice()
    }

    /// Returns validated model rows.
    #[must_use]
    pub fn models(&self) -> &[ModelCatalogEntry] {
        self.models.as_slice()
    }

    /// Returns the deterministic catalog identity.
    #[must_use]
    pub fn catalog_hash(&self) -> &str {
        self.catalog_hash.as_str()
    }
}

/// Exact selected catalog entry and connection class.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedModelCatalogEntry {
    entry: ModelCatalogEntry,
    connection_class: ModelCatalogConnectionClass,
    catalog_hash: String,
}

impl SelectedModelCatalogEntry {
    /// Returns the exact selected entry.
    #[must_use]
    pub const fn entry(&self) -> &ModelCatalogEntry {
        &self.entry
    }

    /// Returns the selected connection class.
    #[must_use]
    pub const fn connection_class(&self) -> ModelCatalogConnectionClass {
        self.connection_class
    }

    /// Returns the catalog identity used for selection.
    #[must_use]
    pub fn catalog_hash(&self) -> &str {
        self.catalog_hash.as_str()
    }
}

/// Merges one shipped catalog with one user catalog by stable model ID.
///
/// Existing stable IDs are replaced as complete entries. New IDs append in
/// validated user order. User connections must be custom and cannot redefine a
/// shipped built-in connection.
///
/// # Errors
///
/// Returns [`ModelCatalogError`] for reserved connections, invalid user
/// connection classes, or invalid merged references.
///
/// # Panics
///
/// Panics if deterministic merged-catalog identity construction returns empty text.
// r[impl animus.catalog.version]
pub fn merge_model_catalogs(
    shipped: &ModelCatalog,
    user: &ModelCatalog,
) -> Result<ModelCatalog, ModelCatalogError> {
    let mut connections = shipped.connections.clone();
    for connection in user.connections() {
        if connection.class() != ModelCatalogConnectionClass::Custom {
            return Err(error(
                ModelCatalogIssue::InvalidUserConnectionClass,
                user.source_label(),
                FIELD_CONNECTIONS,
            ));
        }
        if shipped.connections().iter().any(|candidate| {
            candidate.id() == connection.id()
                && candidate.class() == ModelCatalogConnectionClass::BuiltIn
        }) {
            return Err(error(
                ModelCatalogIssue::ReservedConnection,
                user.source_label(),
                FIELD_CONNECTIONS,
            ));
        }
        if connections
            .iter()
            .any(|candidate| candidate.id() == connection.id())
        {
            return Err(error(
                ModelCatalogIssue::DuplicateIdentity,
                user.source_label(),
                FIELD_CONNECTIONS,
            ));
        }
        connections.push(connection.clone());
    }

    let mut models = shipped.models.clone();
    for entry in user.models() {
        if let Some(existing) = models
            .iter_mut()
            .find(|candidate| candidate.id() == entry.id())
        {
            *existing = entry.clone();
        } else {
            models.push(entry.clone());
        }
    }
    validate_owned_model_references(
        models.as_slice(),
        connections.as_slice(),
        user.source_label(),
    )?;
    let catalog = ModelCatalog {
        source_label: format!("{}+{}", shipped.source_label(), user.source_label()),
        catalog_hash: catalog_hash(connections.as_slice(), models.as_slice()),
        connections,
        models,
    };
    assert!(!catalog.catalog_hash().is_empty());
    Ok(catalog)
}

/// Selects one exact stable model ID under manual or automatic policy.
///
/// # Errors
///
/// Returns [`ModelCatalogError`] when the ID is unavailable or automatic
/// selection reaches a custom connection.
// r[impl animus.catalog.selection]
pub fn select_model_catalog_entry(
    catalog: &ModelCatalog,
    model_id: &super::ModelId,
    mode: ModelCatalogSelectionMode,
) -> Result<SelectedModelCatalogEntry, ModelCatalogError> {
    let entry = catalog
        .models()
        .iter()
        .find(|entry| entry.id() == model_id)
        .ok_or_else(|| {
            error(
                ModelCatalogIssue::ModelUnavailable,
                catalog.source_label(),
                FIELD_MODELS,
            )
        })?;
    let connection = catalog
        .connections()
        .iter()
        .find(|connection| connection.id() == entry.connection_id())
        .ok_or_else(|| {
            error(
                ModelCatalogIssue::MissingConnection,
                catalog.source_label(),
                FIELD_CONNECTIONS,
            )
        })?;
    if mode == ModelCatalogSelectionMode::Automatic
        && connection.class() == ModelCatalogConnectionClass::Custom
    {
        return Err(error(
            ModelCatalogIssue::AutomaticCustomConnection,
            catalog.source_label(),
            FIELD_SELECTION,
        ));
    }
    Ok(SelectedModelCatalogEntry {
        entry: entry.clone(),
        connection_class: connection.class(),
        catalog_hash: catalog.catalog_hash().to_string(),
    })
}

/// Parses one complete user or shipped catalog from JSON.
///
/// # Errors
///
/// Returns a safe [`ModelCatalogError`] that names only the supplied source
/// label and stable field class. Literal credential values are never echoed.
// r[impl animus.catalog.failure]
pub fn parse_model_catalog_json(
    source_label: &str,
    input: &str,
) -> Result<ModelCatalog, ModelCatalogError> {
    let safe_source = validate_source_label(source_label)?;
    let generated: JsonCatalog = serde_json::from_str(input)
        .map_err(|_error| error(ModelCatalogIssue::InvalidJson, safe_source.as_str(), "json"))?;
    let connection_drafts: Vec<ModelCatalogConnectionDraft<'_>> = generated
        .connections
        .iter()
        .map(|connection| {
            parse_connection_class(connection.class.as_str(), safe_source.as_str()).map(|class| {
                ModelCatalogConnectionDraft {
                    id: connection.id.as_str(),
                    class,
                    credential_env: connection
                        .credential_env
                        .as_deref()
                        .filter(|value| !value.is_empty()),
                    credential_value: connection.credential_value.as_deref(),
                }
            })
        })
        .collect::<Result<_, _>>()?;
    let model_drafts: Vec<ModelCatalogEntryDraft<'_>> = generated
        .models
        .iter()
        .map(|entry| ModelCatalogEntryDraft {
            id: entry.id.as_str(),
            connection_id: entry.connection_id.as_str(),
            provider_id: entry.provider_id.as_str(),
            model_id: entry.model_id.as_str(),
        })
        .collect();
    ModelCatalog::validate(&ModelCatalogDraft {
        schema: generated.schema.as_str(),
        source_label: safe_source.as_str(),
        connections: connection_drafts.as_slice(),
        models: model_drafts.as_slice(),
    })
}

/// Loads one complete model catalog from an explicit path.
///
/// # Errors
///
/// Returns a safe [`ModelCatalogError`] for read, JSON, or validation failure.
pub fn load_model_catalog(
    path: impl AsRef<std::path::Path>,
) -> Result<ModelCatalog, ModelCatalogError> {
    let path = path.as_ref();
    let source_label = path
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("model-catalog");
    let safe_source =
        validate_source_label(source_label).unwrap_or_else(|_error| String::from("model-catalog"));
    let input = std::fs::read_to_string(path).map_err(|_error| {
        error(
            ModelCatalogIssue::ReadFailed,
            safe_source.as_str(),
            FIELD_SOURCE,
        )
    })?;
    parse_model_catalog_json(safe_source.as_str(), input.as_str())
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct JsonCatalog {
    schema: String,
    connections: Vec<JsonConnection>,
    models: Vec<JsonModel>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct JsonConnection {
    id: String,
    class: String,
    credential_env: Option<String>,
    credential_value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct JsonModel {
    id: String,
    connection_id: String,
    provider_id: String,
    model_id: String,
}

fn parse_connection_class(
    value: &str,
    source_label: &str,
) -> Result<ModelCatalogConnectionClass, ModelCatalogError> {
    match value {
        "built_in" => Ok(ModelCatalogConnectionClass::BuiltIn),
        "custom" => Ok(ModelCatalogConnectionClass::Custom),
        _ => Err(error(
            ModelCatalogIssue::InvalidIdentifier,
            source_label,
            FIELD_CONNECTIONS,
        )),
    }
}

fn validate_connections(
    drafts: &[ModelCatalogConnectionDraft<'_>],
    source_label: &str,
) -> Result<Vec<ModelCatalogConnection>, ModelCatalogError> {
    let mut connections = Vec::with_capacity(drafts.len());
    for draft in drafts {
        if draft
            .credential_value
            .is_some_and(|value| !value.is_empty())
        {
            return Err(error(
                ModelCatalogIssue::LiteralCredential,
                source_label,
                FIELD_CREDENTIAL_VALUE,
            ));
        }
        let id = super::ProviderId::new(draft.id).map_err(|_error| {
            error(
                ModelCatalogIssue::InvalidIdentifier,
                source_label,
                FIELD_CONNECTIONS,
            )
        })?;
        if connections
            .iter()
            .any(|connection: &ModelCatalogConnection| connection.id() == &id)
        {
            return Err(error(
                ModelCatalogIssue::DuplicateIdentity,
                source_label,
                FIELD_CONNECTIONS,
            ));
        }
        let credential_env = draft
            .credential_env
            .map(|name| {
                super::CredentialSource::environment(name)
                    .map(|_source| String::from(name))
                    .map_err(|_error| {
                        error(
                            ModelCatalogIssue::InvalidIdentifier,
                            source_label,
                            FIELD_CREDENTIAL_ENV,
                        )
                    })
            })
            .transpose()?;
        connections.push(ModelCatalogConnection {
            id,
            class: draft.class,
            credential_env,
        });
    }
    Ok(connections)
}

fn validate_models(
    drafts: &[ModelCatalogEntryDraft<'_>],
    connections: &[ModelCatalogConnection],
    source_label: &str,
) -> Result<Vec<ModelCatalogEntry>, ModelCatalogError> {
    let mut models = Vec::with_capacity(drafts.len());
    for draft in drafts {
        let entry = ModelCatalogEntry {
            id: super::ModelId::new(draft.id).map_err(|_error| {
                error(
                    ModelCatalogIssue::InvalidIdentifier,
                    source_label,
                    FIELD_MODELS,
                )
            })?,
            connection_id: super::ProviderId::new(draft.connection_id).map_err(|_error| {
                error(
                    ModelCatalogIssue::InvalidIdentifier,
                    source_label,
                    FIELD_MODELS,
                )
            })?,
            provider_id: super::ProviderId::new(draft.provider_id).map_err(|_error| {
                error(
                    ModelCatalogIssue::InvalidIdentifier,
                    source_label,
                    FIELD_MODELS,
                )
            })?,
            model_id: super::ModelId::new(draft.model_id).map_err(|_error| {
                error(
                    ModelCatalogIssue::InvalidIdentifier,
                    source_label,
                    FIELD_MODELS,
                )
            })?,
        };
        if models
            .iter()
            .any(|candidate: &ModelCatalogEntry| candidate.id() == entry.id())
        {
            return Err(error(
                ModelCatalogIssue::DuplicateIdentity,
                source_label,
                FIELD_MODELS,
            ));
        }
        if !connections
            .iter()
            .any(|connection| connection.id() == entry.connection_id())
        {
            return Err(error(
                ModelCatalogIssue::MissingConnection,
                source_label,
                FIELD_MODELS,
            ));
        }
        models.push(entry);
    }
    Ok(models)
}

fn validate_owned_model_references(
    models: &[ModelCatalogEntry],
    connections: &[ModelCatalogConnection],
    source_label: &str,
) -> Result<(), ModelCatalogError> {
    if models.iter().all(|entry| {
        connections
            .iter()
            .any(|connection| connection.id() == entry.connection_id())
    }) {
        return Ok(());
    }
    Err(error(
        ModelCatalogIssue::MissingConnection,
        source_label,
        FIELD_MODELS,
    ))
}

fn validate_collection(
    count: usize,
    maximum: usize,
    source_label: &str,
    field_class: &'static str,
) -> Result<(), ModelCatalogError> {
    if count == 0 || count > maximum {
        return Err(error(
            ModelCatalogIssue::InvalidCollection,
            source_label,
            field_class,
        ));
    }
    Ok(())
}

fn validate_source_label(source_label: &str) -> Result<String, ModelCatalogError> {
    super::ProviderId::new(source_label)
        .map(|_id| String::from(source_label))
        .map_err(|_error| {
            error(
                ModelCatalogIssue::InvalidIdentifier,
                "model-catalog",
                FIELD_SOURCE,
            )
        })
}

fn catalog_hash(connections: &[ModelCatalogConnection], models: &[ModelCatalogEntry]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(CATALOG_HASH_DOMAIN.as_bytes());
    for connection in connections {
        update_hash(&mut hasher, connection.id().as_str());
        update_hash(&mut hasher, connection.class().as_str());
        update_hash(&mut hasher, connection.credential_env().unwrap_or("none"));
    }
    for entry in models {
        update_hash(&mut hasher, entry.id().as_str());
        update_hash(&mut hasher, entry.connection_id().as_str());
        update_hash(&mut hasher, entry.provider_id().as_str());
        update_hash(&mut hasher, entry.model_id().as_str());
    }
    hasher.finalize().to_hex().to_string()
}

fn update_hash(hasher: &mut blake3::Hasher, value: &str) {
    hasher.update(HASH_SEPARATOR.as_bytes());
    hasher.update(value.as_bytes());
}

fn error(
    issue: ModelCatalogIssue,
    source_label: &str,
    field_class: &'static str,
) -> ModelCatalogError {
    ModelCatalogError {
        issue,
        source_label: String::from(source_label),
        field_class,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GENERATED: &str = include_str!("../../../../config/generated/model-catalog.json");
    const SHIPPED: &str = r#"{
      "schema": "animus.model-catalog.v1",
      "connections": [
        {"id":"openai","class":"built_in","credential_env":"OPENAI_API_KEY","credential_value":null}
      ],
      "models": [
        {"id":"primary","connection_id":"openai","provider_id":"openai","model_id":"gpt-primary"},
        {"id":"retained","connection_id":"openai","provider_id":"openai","model_id":"gpt-retained"}
      ]
    }"#;
    const USER: &str = r#"{
      "schema": "animus.model-catalog.v1",
      "connections": [
        {"id":"local-gateway","class":"custom","credential_env":null,"credential_value":null}
      ],
      "models": [
        {"id":"primary","connection_id":"local-gateway","provider_id":"local","model_id":"qwen-local"},
        {"id":"new-local","connection_id":"local-gateway","provider_id":"local","model_id":"qwen-new"}
      ]
    }"#;

    #[test]
    fn merge_replaces_by_stable_id_and_manual_custom_selection_succeeds() {
        // r[verify animus.catalog.version.merge]
        // r[verify animus.catalog.secret.reference]
        // r[verify animus.catalog.selection.manual]
        // r[verify animus.catalog.verification]
        // r[verify animus.catalog.verification.matrix]
        let generated =
            parse_model_catalog_json("generated", GENERATED).expect("generated checked catalog");
        assert_eq!(generated.models()[0].id().as_str(), "mock-text");
        let shipped = parse_model_catalog_json("shipped", SHIPPED).expect("shipped catalog");
        let user = parse_model_catalog_json("user", USER).expect("user catalog");
        let merged = merge_model_catalogs(&shipped, &user).expect("merged catalog");

        assert_eq!(merged.models().len(), 3);
        assert_eq!(merged.models()[0].id().as_str(), "primary");
        assert_eq!(merged.models()[0].connection_id().as_str(), "local-gateway");
        assert_eq!(merged.models()[1].id().as_str(), "retained");
        assert_eq!(merged.models()[2].id().as_str(), "new-local");
        let selected = select_model_catalog_entry(
            &merged,
            &super::super::ModelId::new("new-local").expect("model id"),
            ModelCatalogSelectionMode::Manual,
        )
        .expect("manual custom selection");
        assert_eq!(
            selected.connection_class(),
            ModelCatalogConnectionClass::Custom
        );
        assert_eq!(selected.catalog_hash(), merged.catalog_hash());
    }

    #[test]
    fn invalid_and_automatic_custom_inputs_fail_without_secret_echo() {
        // r[verify animus.catalog.version.duplicate]
        // r[verify animus.catalog.secret.value]
        // r[verify animus.catalog.failure.field]
        // r[verify animus.catalog.selection.fallback]
        let shipped = parse_model_catalog_json("shipped", SHIPPED).expect("shipped catalog");
        let user = parse_model_catalog_json("user", USER).expect("user catalog");
        let merged = merge_model_catalogs(&shipped, &user).expect("merged catalog");
        let automatic = select_model_catalog_entry(
            &merged,
            &super::super::ModelId::new("new-local").expect("model id"),
            ModelCatalogSelectionMode::Automatic,
        )
        .expect_err("custom automatic fallback");
        assert_eq!(
            automatic.issue(),
            ModelCatalogIssue::AutomaticCustomConnection
        );

        let literal = USER.replace(
            "\"credential_value\":null",
            "\"credential_value\":\"top-secret-value\"",
        );
        let literal_error =
            parse_model_catalog_json("user", literal.as_str()).expect_err("literal credential");
        assert_eq!(literal_error.issue(), ModelCatalogIssue::LiteralCredential);
        assert!(!literal_error.to_string().contains("top-secret-value"));

        let malformed =
            parse_model_catalog_json("user-file", "{not-json").expect_err("malformed JSON");
        assert_eq!(malformed.issue(), ModelCatalogIssue::InvalidJson);
        assert_eq!(malformed.source_label(), "user-file");
        assert_eq!(malformed.field_class(), "json");
    }

    #[test]
    fn reserved_duplicate_and_unsupported_sources_fail_closed() {
        // r[verify animus.catalog.version.duplicate]
        let duplicate = SHIPPED.replace(
            "{\"id\":\"retained\",\"connection_id\":\"openai\",\"provider_id\":\"openai\",\"model_id\":\"gpt-retained\"}",
            "{\"id\":\"primary\",\"connection_id\":\"openai\",\"provider_id\":\"openai\",\"model_id\":\"gpt-retained\"}",
        );
        let duplicate_error = parse_model_catalog_json("shipped", duplicate.as_str())
            .expect_err("duplicate stable ID");
        assert_eq!(
            duplicate_error.issue(),
            ModelCatalogIssue::DuplicateIdentity
        );

        let unsupported = SHIPPED.replace(MODEL_CATALOG_SCHEMA, "animus.model-catalog.v0");
        let unsupported_error = parse_model_catalog_json("shipped", unsupported.as_str())
            .expect_err("unsupported schema");
        assert_eq!(
            unsupported_error.issue(),
            ModelCatalogIssue::UnsupportedSchema
        );

        let reserved_user = r#"{
          "schema":"animus.model-catalog.v1",
          "connections":[{"id":"openai","class":"custom","credential_env":null,"credential_value":null}],
          "models":[{"id":"custom","connection_id":"openai","provider_id":"openai","model_id":"custom-model"}]
        }"#;
        let reserved = parse_model_catalog_json("user", reserved_user).expect("user catalog");
        let shipped = parse_model_catalog_json("shipped", SHIPPED).expect("shipped catalog");
        let reserved_error =
            merge_model_catalogs(&shipped, &reserved).expect_err("reserved connection");
        assert_eq!(
            reserved_error.issue(),
            ModelCatalogIssue::ReservedConnection
        );

        let built_in_user = USER.replace("\"class\":\"custom\"", "\"class\":\"built_in\"");
        let built_in_user =
            parse_model_catalog_json("user", built_in_user.as_str()).expect("typed user catalog");
        let class_error = merge_model_catalogs(&shipped, &built_in_user)
            .expect_err("user built-in connection class");
        assert_eq!(
            class_error.issue(),
            ModelCatalogIssue::InvalidUserConnectionClass
        );
    }
}
