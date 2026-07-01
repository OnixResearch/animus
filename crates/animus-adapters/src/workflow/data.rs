use super::output::{RecipeResponseSchema, RecipeResponseSchemaDraft};
use super::retry::{RecipeRetryPolicy, RecipeRetryPolicyDraft};
use super::trust::{RecipeProvenanceDraft, RecipeTrustReceipt};

const IDENTITY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_workflow_identity");
const INSTRUCTIONS_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_workflow_instructions");
const PROMPT_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_workflow_initial_prompt");
const ACTIVITY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_workflow_activity");
const PARAMETER_NAME_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_workflow_parameter_name");
const PARAMETER_KIND_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_workflow_parameter_kind");
const PARAMETER_DEFAULT_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_workflow_parameter_default");
const PARAMETER_OPTION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_workflow_parameter_option");
const MODEL_PROFILE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_workflow_model_profile");
const CAPABILITY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_workflow_capability");
const TOOLSET_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_workflow_toolset");
const GATE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_workflow_validation_gate");
const SUBRECIPE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("recipe_workflow_subrecipe");

const KIND_TEXT: &str = "text";
const KIND_SELECT: &str = "select";
const KIND_FILE: &str = "file";

type Text = alloc::string::String;
type Collection<T> = alloc::vec::Vec<T>;

/// Untrusted recipe/workflow manifest fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecipeWorkflowDraft<'a> {
    /// Stable recipe identity.
    pub identity: &'a str,
    /// Instruction block used by prompt assembly after policy admission.
    pub instructions: &'a str,
    /// Initial prompt template before parameter binding.
    pub initial_prompt: &'a str,
    /// Ordered activity labels for the workflow.
    pub activities: &'a [&'a str],
    /// Declared input parameters.
    pub parameters: &'a [RecipeParameterDraft<'a>],
    /// Model profile label requested by the recipe.
    pub model_profile: &'a str,
    /// Capability ceiling labels the recipe may request.
    pub capability_ceilings: &'a [&'a str],
    /// Toolset reference labels declared by the recipe.
    pub toolsets: &'a [&'a str],
    /// Validation gate labels that must pass before success is reported.
    pub validation_gates: &'a [&'a str],
    /// Host-mediated retry policy metadata.
    pub retry_policy: RecipeRetryPolicyDraft<'a>,
    /// Referenced subrecipe identities.
    pub subrecipes: &'a [&'a str],
    /// Structured final response schema.
    pub response_schema: RecipeResponseSchemaDraft<'a>,
    /// Trust and provenance metadata.
    pub provenance: RecipeProvenanceDraft<'a>,
}

/// Untrusted recipe parameter declaration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecipeParameterDraft<'a> {
    /// Stable parameter name.
    pub name: &'a str,
    /// Parameter kind label.
    pub kind: &'a str,
    /// Whether a caller must provide this parameter explicitly.
    pub required: bool,
    /// Optional default value for non-file optional parameters.
    pub default_value: Option<&'a str>,
    /// Select-only allowed values.
    pub select_options: &'a [&'a str],
}

/// Validated recipe workflow manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecipeWorkflow {
    identity: Text,
    instructions: Text,
    initial_prompt: Text,
    activities: Collection<Text>,
    parameters: Collection<RecipeParameter>,
    model_profile: Text,
    capability_ceilings: Collection<Text>,
    toolsets: Collection<Text>,
    validation_gates: Collection<Text>,
    retry_policy: RecipeRetryPolicy,
    subrecipes: Collection<Text>,
    response_schema: RecipeResponseSchema,
    provenance: RecipeTrustReceipt,
}

impl RecipeWorkflow {
    /// Validates untrusted recipe workflow metadata without I/O or effects.
    ///
    /// # Errors
    ///
    /// Returns [`super::RecipeWorkflowError`] when required fields are empty,
    /// collections are invalid, parameters are unsafe, retry policy is
    /// unbounded, structured output schema is malformed, or provenance is not a
    /// valid BLAKE3-identified trust receipt.
    pub fn validate(draft: &RecipeWorkflowDraft<'_>) -> Result<Self, super::RecipeWorkflowError> {
        let identity = super::required_text(IDENTITY_FIELD, draft.identity)?;
        let instructions = super::required_text(INSTRUCTIONS_FIELD, draft.instructions)?;
        let initial_prompt = super::required_text(PROMPT_FIELD, draft.initial_prompt)?;
        let activities = super::text_collection(ACTIVITY_FIELD, draft.activities, true)?;
        let parameters = collect_parameters(draft.parameters)?;
        let model_profile = super::required_text(MODEL_PROFILE_FIELD, draft.model_profile)?;
        let capability_ceilings =
            super::text_collection(CAPABILITY_FIELD, draft.capability_ceilings, true)?;
        let toolsets = super::text_collection(TOOLSET_FIELD, draft.toolsets, false)?;
        let validation_gates = super::text_collection(GATE_FIELD, draft.validation_gates, true)?;
        let retry_policy = RecipeRetryPolicy::validate(&draft.retry_policy)?;
        let subrecipes = super::text_collection(SUBRECIPE_FIELD, draft.subrecipes, false)?;
        let response_schema = RecipeResponseSchema::validate(&draft.response_schema)?;
        let provenance = RecipeTrustReceipt::new(&draft.provenance)?;

        Ok(Self {
            identity,
            instructions,
            initial_prompt,
            activities,
            parameters,
            model_profile,
            capability_ceilings,
            toolsets,
            validation_gates,
            retry_policy,
            subrecipes,
            response_schema,
            provenance,
        })
    }

    /// Returns the stable recipe identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }

    /// Returns the instruction block.
    #[must_use]
    pub fn instructions(&self) -> &str {
        self.instructions.as_str()
    }

    /// Returns the initial prompt template.
    #[must_use]
    pub fn initial_prompt(&self) -> &str {
        self.initial_prompt.as_str()
    }

    /// Returns ordered activity labels.
    #[must_use]
    pub fn activities(&self) -> &[Text] {
        self.activities.as_slice()
    }

    /// Returns validated parameter declarations.
    #[must_use]
    pub fn parameters(&self) -> &[RecipeParameter] {
        self.parameters.as_slice()
    }

    /// Returns the requested model profile label.
    #[must_use]
    pub fn model_profile(&self) -> &str {
        self.model_profile.as_str()
    }

    /// Returns capability ceiling labels.
    #[must_use]
    pub fn capability_ceilings(&self) -> &[Text] {
        self.capability_ceilings.as_slice()
    }

    /// Returns declared toolset references.
    #[must_use]
    pub fn toolsets(&self) -> &[Text] {
        self.toolsets.as_slice()
    }

    /// Returns validation gate labels.
    #[must_use]
    pub fn validation_gates(&self) -> &[Text] {
        self.validation_gates.as_slice()
    }

    /// Returns the host-mediated retry policy.
    #[must_use]
    pub const fn retry_policy(&self) -> &RecipeRetryPolicy {
        &self.retry_policy
    }

    /// Returns referenced subrecipe identities.
    #[must_use]
    pub fn subrecipes(&self) -> &[Text] {
        self.subrecipes.as_slice()
    }

    /// Returns the structured final response schema.
    #[must_use]
    pub const fn response_schema(&self) -> &RecipeResponseSchema {
        &self.response_schema
    }

    /// Returns the recipe trust receipt.
    #[must_use]
    pub const fn provenance(&self) -> &RecipeTrustReceipt {
        &self.provenance
    }
}

/// Supported recipe parameter kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecipeParameterKind {
    /// Plain literal text supplied by the caller or default.
    Text,
    /// Literal text constrained to declared options.
    Select,
    /// Host-provided file content reference, never raw default content.
    File,
}

impl RecipeParameterKind {
    /// Parses a stable parameter kind label.
    ///
    /// # Errors
    ///
    /// Returns [`super::RecipeWorkflowError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, super::RecipeWorkflowError> {
        super::parse_workflow_value(PARAMETER_KIND_FIELD, value, Self::from_normalized)
    }

    fn from_normalized(value: &str) -> Option<Self> {
        match value {
            KIND_TEXT => Some(Self::Text),
            KIND_SELECT => Some(Self::Select),
            KIND_FILE => Some(Self::File),
            _ => None,
        }
    }

    /// Returns the stable parameter kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => KIND_TEXT,
            Self::Select => KIND_SELECT,
            Self::File => KIND_FILE,
        }
    }
}

/// Validated recipe parameter declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecipeParameter {
    name: Text,
    kind: RecipeParameterKind,
    required: bool,
    default_value: Option<Text>,
    select_options: Collection<Text>,
}

impl RecipeParameter {
    fn validate(draft: &RecipeParameterDraft<'_>) -> Result<Self, super::RecipeWorkflowError> {
        let name = super::required_text(PARAMETER_NAME_FIELD, draft.name)?;
        let kind = RecipeParameterKind::parse(draft.kind)?;
        let default_value = super::optional_text(PARAMETER_DEFAULT_FIELD, draft.default_value)?;
        let select_options =
            super::text_collection(PARAMETER_OPTION_FIELD, draft.select_options, false)?;
        validate_parameter_defaults(name.as_str(), kind, draft.required, default_value.as_ref())?;
        validate_parameter_options(name.as_str(), kind, default_value.as_ref(), &select_options)?;

        Ok(Self {
            name,
            kind,
            required: draft.required,
            default_value,
            select_options,
        })
    }

    /// Returns the stable parameter name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the parameter kind.
    #[must_use]
    pub const fn kind(&self) -> RecipeParameterKind {
        self.kind
    }

    /// Returns whether the caller must supply this parameter explicitly.
    #[must_use]
    pub const fn required(&self) -> bool {
        self.required
    }

    /// Returns the validated default value, when any.
    #[must_use]
    pub fn default_value(&self) -> Option<&str> {
        self.default_value.as_deref()
    }

    /// Returns declared select options.
    #[must_use]
    pub fn select_options(&self) -> &[Text] {
        self.select_options.as_slice()
    }
}

fn collect_parameters(
    drafts: &[RecipeParameterDraft<'_>],
) -> Result<Collection<RecipeParameter>, super::RecipeWorkflowError> {
    let mut parameters: Collection<RecipeParameter> = Collection::with_capacity(drafts.len());
    for draft in drafts {
        let parameter = RecipeParameter::validate(draft)?;
        if parameters
            .iter()
            .any(|existing| existing.name() == parameter.name())
        {
            return Err(super::RecipeWorkflowError::DuplicateName {
                field: PARAMETER_NAME_FIELD.as_str(),
                value: Text::from(parameter.name()),
            });
        }
        parameters.push(parameter);
    }
    Ok(parameters)
}

fn validate_parameter_defaults(
    name: &str,
    kind: RecipeParameterKind,
    required: bool,
    default_value: Option<&Text>,
) -> Result<(), super::RecipeWorkflowError> {
    let parameter = Text::from(name);
    if kind == RecipeParameterKind::File && default_value.is_some() {
        return Err(super::RecipeWorkflowError::FileParameterDefault { parameter });
    }
    if kind == RecipeParameterKind::File && !required {
        return Err(super::RecipeWorkflowError::OptionalFileParameter { parameter });
    }
    if !required && default_value.is_none() {
        return Err(super::RecipeWorkflowError::MissingOptionalDefault { parameter });
    }
    Ok(())
}

fn validate_parameter_options(
    name: &str,
    kind: RecipeParameterKind,
    default_value: Option<&Text>,
    select_options: &[Text],
) -> Result<(), super::RecipeWorkflowError> {
    let parameter = Text::from(name);
    if kind == RecipeParameterKind::Select && select_options.is_empty() {
        return Err(super::RecipeWorkflowError::EmptySelectOptions { parameter });
    }
    if kind != RecipeParameterKind::Select && !select_options.is_empty() {
        return Err(super::RecipeWorkflowError::UnexpectedSelectOptions { parameter });
    }
    if let Some(default_text) = default_value {
        if kind == RecipeParameterKind::Select
            && !super::contains_text(select_options, default_text.as_str())
        {
            return Err(super::RecipeWorkflowError::InvalidSelectValue {
                parameter,
                value: default_text.clone(),
            });
        }
    }
    Ok(())
}
