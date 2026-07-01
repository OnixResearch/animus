const SCHEMA_NAME_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_response_schema_name");
const FIELD_NAME_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_response_field_name");
const FIELD_KIND_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_response_field_kind");

const KIND_STRING: &str = "string";
const KIND_NUMBER: &str = "number";
const KIND_BOOL: &str = "bool";
const KIND_OBJECT: &str = "object";
const KIND_ARRAY: &str = "array";

type Text = alloc::string::String;
type Collection<T> = alloc::vec::Vec<T>;

/// Untrusted structured response schema.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecipeResponseSchemaDraft<'a> {
    /// Stable schema name.
    pub name: &'a str,
    /// Required fields for successful automation output.
    pub required_fields: &'a [RecipeResponseFieldDraft<'a>],
}

/// Untrusted response field declaration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecipeResponseFieldDraft<'a> {
    /// Field name.
    pub name: &'a str,
    /// Field kind label.
    pub kind: &'a str,
}

/// Untrusted observed final output facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecipeOutputDraft<'a> {
    /// Observed top-level fields and their kinds.
    pub fields: &'a [RecipeOutputFieldDraft<'a>],
}

/// Untrusted observed final output field fact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecipeOutputFieldDraft<'a> {
    /// Observed field name.
    pub name: &'a str,
    /// Observed field kind label.
    pub kind: &'a str,
}

/// Supported structured response field kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecipeResponseFieldKind {
    /// JSON string value.
    String,
    /// JSON number value.
    Number,
    /// JSON boolean value.
    Bool,
    /// JSON object value.
    Object,
    /// JSON array value.
    Array,
}

impl RecipeResponseFieldKind {
    /// Parses a stable response field kind label.
    ///
    /// # Errors
    ///
    /// Returns [`super::RecipeWorkflowError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, super::RecipeWorkflowError> {
        super::parse_workflow_value(FIELD_KIND_FIELD, value, Self::from_normalized)
    }

    fn from_normalized(value: &str) -> Option<Self> {
        match value {
            KIND_STRING => Some(Self::String),
            KIND_NUMBER => Some(Self::Number),
            KIND_BOOL => Some(Self::Bool),
            KIND_OBJECT => Some(Self::Object),
            KIND_ARRAY => Some(Self::Array),
            _ => None,
        }
    }

    /// Returns the stable response field kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::String => KIND_STRING,
            Self::Number => KIND_NUMBER,
            Self::Bool => KIND_BOOL,
            Self::Object => KIND_OBJECT,
            Self::Array => KIND_ARRAY,
        }
    }
}

/// Validated structured response schema.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecipeResponseSchema {
    name: Text,
    required_fields: Collection<RecipeResponseField>,
}

impl RecipeResponseSchema {
    /// Validates a final response schema.
    ///
    /// # Errors
    ///
    /// Returns [`super::RecipeWorkflowError`] when the schema is unnamed,
    /// declares no required fields, repeats a field, or names an unsupported kind.
    pub fn validate(
        draft: &RecipeResponseSchemaDraft<'_>,
    ) -> Result<Self, super::RecipeWorkflowError> {
        let name = super::required_text(SCHEMA_NAME_FIELD, draft.name)?;
        if draft.required_fields.is_empty() {
            return Err(super::RecipeWorkflowError::EmptyCollection {
                field: FIELD_NAME_FIELD.as_str(),
            });
        }
        let required_fields = collect_fields(draft.required_fields)?;

        Ok(Self {
            name,
            required_fields,
        })
    }

    /// Returns the stable schema name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns required output fields.
    #[must_use]
    pub fn required_fields(&self) -> &[RecipeResponseField] {
        self.required_fields.as_slice()
    }
}

/// Validated response field declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecipeResponseField {
    name: Text,
    kind: RecipeResponseFieldKind,
}

impl RecipeResponseField {
    fn validate(draft: &RecipeResponseFieldDraft<'_>) -> Result<Self, super::RecipeWorkflowError> {
        let name = super::required_text(FIELD_NAME_FIELD, draft.name)?;
        let kind = RecipeResponseFieldKind::parse(draft.kind)?;
        Ok(Self { name, kind })
    }

    /// Returns the field name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the field kind.
    #[must_use]
    pub const fn kind(&self) -> RecipeResponseFieldKind {
        self.kind
    }
}

/// Result status for structured automation output validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecipeAutomationStatus {
    /// Final output satisfied the declared schema.
    Success,
    /// Final output failed schema validation and must not be consumed as success.
    StructuredOutputFailure,
}

/// Structured output validation failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecipeOutputFailure {
    /// A required field was absent.
    MissingField {
        /// Missing field name.
        field: Text,
    },
    /// A required field had the wrong kind.
    KindMismatch {
        /// Field name.
        field: Text,
        /// Expected field kind.
        expected: RecipeResponseFieldKind,
        /// Actual observed field kind.
        actual: RecipeResponseFieldKind,
    },
}

/// Automation outcome recorded before an external caller consumes recipe output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecipeAutomationOutcome {
    status: RecipeAutomationStatus,
    failure: Option<RecipeOutputFailure>,
}

impl RecipeAutomationOutcome {
    fn success() -> Self {
        Self {
            status: RecipeAutomationStatus::Success,
            failure: None,
        }
    }

    fn failure(failure: RecipeOutputFailure) -> Self {
        Self {
            status: RecipeAutomationStatus::StructuredOutputFailure,
            failure: Some(failure),
        }
    }

    /// Returns the automation validation status.
    #[must_use]
    pub const fn status(&self) -> RecipeAutomationStatus {
        self.status
    }

    /// Returns the structured validation failure, when any.
    #[must_use]
    pub const fn failure_reason(&self) -> Option<&RecipeOutputFailure> {
        self.failure.as_ref()
    }
}

/// Validates observed output facts against the declared final response schema.
///
/// # Errors
///
/// Returns [`super::RecipeWorkflowError`] when observed field labels are malformed
/// or repeated. Missing or wrong-typed fields produce a failure outcome instead
/// of an error so automation can record the denial explicitly.
pub fn validate_recipe_output(
    schema: &RecipeResponseSchema,
    output: &RecipeOutputDraft<'_>,
) -> Result<RecipeAutomationOutcome, super::RecipeWorkflowError> {
    let observed = collect_output_fields(output.fields)?;
    for required in schema.required_fields() {
        match find_field(observed.as_slice(), required.name()) {
            None => {
                return Ok(RecipeAutomationOutcome::failure(
                    RecipeOutputFailure::MissingField {
                        field: Text::from(required.name()),
                    },
                ));
            }
            Some(actual) if actual.kind != required.kind() => {
                return Ok(RecipeAutomationOutcome::failure(
                    RecipeOutputFailure::KindMismatch {
                        field: Text::from(required.name()),
                        expected: required.kind(),
                        actual: actual.kind,
                    },
                ));
            }
            Some(_actual) => {}
        }
    }
    Ok(RecipeAutomationOutcome::success())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ObservedField {
    name: Text,
    kind: RecipeResponseFieldKind,
}

fn collect_fields(
    drafts: &[RecipeResponseFieldDraft<'_>],
) -> Result<Collection<RecipeResponseField>, super::RecipeWorkflowError> {
    let mut fields: Collection<RecipeResponseField> = Collection::with_capacity(drafts.len());
    for draft in drafts {
        let field = RecipeResponseField::validate(draft)?;
        if fields
            .iter()
            .any(|existing| existing.name() == field.name())
        {
            return Err(super::RecipeWorkflowError::DuplicateName {
                field: FIELD_NAME_FIELD.as_str(),
                value: Text::from(field.name()),
            });
        }
        fields.push(field);
    }
    Ok(fields)
}

fn collect_output_fields(
    drafts: &[RecipeOutputFieldDraft<'_>],
) -> Result<Collection<ObservedField>, super::RecipeWorkflowError> {
    let mut fields: Collection<ObservedField> = Collection::with_capacity(drafts.len());
    for draft in drafts {
        let name = super::required_text(FIELD_NAME_FIELD, draft.name)?;
        let kind = RecipeResponseFieldKind::parse(draft.kind)?;
        if fields.iter().any(|existing| existing.name.as_str() == name) {
            return Err(super::RecipeWorkflowError::DuplicateName {
                field: FIELD_NAME_FIELD.as_str(),
                value: name,
            });
        }
        fields.push(ObservedField { name, kind });
    }
    Ok(fields)
}

fn find_field<'a>(fields: &'a [ObservedField], name: &str) -> Option<&'a ObservedField> {
    fields.iter().find(|field| field.name.as_str() == name)
}
