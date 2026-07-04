//! Animus-owned recipe workflow DTOs and pure validation helpers.
//!
//! These contracts adapt reusable workflow recipes without adopting any external
//! file format as runtime authority. Hosts may import or export YAML/JSON, but
//! this module treats all data as untrusted DTOs and performs no I/O or effects.

mod admit;
mod assign;
mod attempt;
mod data;
mod field;
mod shape;

#[cfg(test)]
mod tests;

pub use admit::{
    RecipeInfluence, RecipeInfluenceStatus, RecipeProvenanceDraft, RecipeSourceKind,
    RecipeTrustReceipt, RecipeTrustStatus,
};
pub use assign::{
    RecipeBinding, RecipeBindingDraft, RecipeParameterInput, RecipeParameterInputDraft,
    RecipeParameterInputKind, SAFE_TEMPLATE_BUILTINS, bind_recipe_parameters,
};
pub use attempt::{
    RecipeHostEffectKind, RecipeRetryCheck, RecipeRetryCheckDraft, RecipeRetryDenialReason,
    RecipeRetryPlan, RecipeRetryPlanDraft, RecipeRetryPolicy, RecipeRetryPolicyDraft,
    RecipeRetryRequest, RecipeRetryRequestStatus, plan_recipe_retry,
};
pub use data::{RecipeWorkflow, RecipeWorkflowDraft};
pub use field::{RecipeParameter, RecipeParameterDraft, RecipeParameterKind};
pub use shape::{
    RecipeAutomationOutcome, RecipeAutomationStatus, RecipeOutputDraft, RecipeOutputFailure,
    RecipeOutputFieldDraft, RecipeResponseField, RecipeResponseFieldDraft, RecipeResponseFieldKind,
    RecipeResponseSchema, RecipeResponseSchemaDraft, validate_recipe_output,
};

mod digest;
/// Error raised by recipe workflow DTO validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecipeWorkflowError {
    /// Generic adapter contract validation failed.
    Contract(crate::AdapterContractError),
    /// A workflow enum-like field contained an unsupported value.
    UnsupportedWorkflowValue {
        /// Boundary field name.
        field: &'static str,
        /// Unsupported normalized value.
        value: alloc::string::String,
    },
    /// A required collection was empty.
    EmptyCollection {
        /// Boundary collection field name.
        field: &'static str,
    },
    /// A named collection contained a duplicate entry.
    DuplicateName {
        /// Boundary collection field name.
        field: &'static str,
        /// Duplicated normalized value.
        value: alloc::string::String,
    },
    /// An optional non-file parameter omitted its required default.
    MissingOptionalDefault {
        /// Parameter name.
        parameter: alloc::string::String,
    },
    /// A file parameter attempted to define a default value.
    FileParameterDefault {
        /// Parameter name.
        parameter: alloc::string::String,
    },
    /// A file parameter was declared optional, which would require an unsafe default.
    OptionalFileParameter {
        /// Parameter name.
        parameter: alloc::string::String,
    },
    /// A select parameter did not declare any allowed option.
    EmptySelectOptions {
        /// Parameter name.
        parameter: alloc::string::String,
    },
    /// A non-select parameter declared select-only options.
    UnexpectedSelectOptions {
        /// Parameter name.
        parameter: alloc::string::String,
    },
    /// A select parameter value was outside its declared options.
    InvalidSelectValue {
        /// Parameter name.
        parameter: alloc::string::String,
        /// Rejected value.
        value: alloc::string::String,
    },
    /// A template marker was not closed.
    MalformedTemplate,
    /// A template referenced a variable that was neither declared nor safe built-in.
    UndeclaredTemplateVariable {
        /// Template variable name.
        variable: alloc::string::String,
    },
    /// A declared parameter was not referenced by the template.
    UnusedParameter {
        /// Parameter name.
        parameter: alloc::string::String,
    },
    /// A required parameter had no supplied value.
    MissingParameterValue {
        /// Parameter name.
        parameter: alloc::string::String,
    },
    /// A parameter value arrived through a source kind that the parameter rejects.
    InvalidParameterSource {
        /// Parameter name.
        parameter: alloc::string::String,
    },
    /// A retry policy declared no bounded attempts.
    ZeroRetryAttempts,
    /// A retry check requested an effect kind that reusable core must never execute.
    UnsafeRetryEffect {
        /// Rejected effect label.
        effect: alloc::string::String,
    },
    /// A BLAKE3 identity field failed lowercase hex validation.
    InvalidHash {
        /// Boundary field name.
        field: &'static str,
    },
}

impl From<crate::AdapterContractError> for RecipeWorkflowError {
    fn from(error: crate::AdapterContractError) -> Self {
        Self::Contract(error)
    }
}

impl core::fmt::Display for RecipeWorkflowError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Contract(error) => core::fmt::Display::fmt(error, formatter),
            Self::UnsupportedWorkflowValue { field, value } => {
                write!(
                    formatter,
                    "{field} contains unsupported workflow value {value}"
                )
            }
            Self::EmptyCollection { field } => write!(formatter, "{field} must not be empty"),
            Self::DuplicateName { field, value } => {
                write!(formatter, "{field} contains duplicate value {value}")
            }
            Self::MissingOptionalDefault { parameter } => {
                write!(
                    formatter,
                    "optional parameter {parameter} must define a default"
                )
            }
            Self::FileParameterDefault { parameter } => {
                write!(
                    formatter,
                    "file parameter {parameter} must not define a default"
                )
            }
            Self::OptionalFileParameter { parameter } => {
                write!(formatter, "file parameter {parameter} must be required")
            }
            Self::EmptySelectOptions { parameter } => {
                write!(
                    formatter,
                    "select parameter {parameter} must define options"
                )
            }
            Self::UnexpectedSelectOptions { parameter } => {
                write!(
                    formatter,
                    "non-select parameter {parameter} must not define options"
                )
            }
            Self::InvalidSelectValue { parameter, value } => {
                write!(
                    formatter,
                    "parameter {parameter} rejects select value {value}"
                )
            }
            Self::MalformedTemplate => formatter.write_str("template marker is not closed"),
            Self::UndeclaredTemplateVariable { variable } => {
                write!(formatter, "template variable {variable} is not declared")
            }
            Self::UnusedParameter { parameter } => {
                write!(
                    formatter,
                    "parameter {parameter} is not referenced by the template"
                )
            }
            Self::MissingParameterValue { parameter } => {
                write!(formatter, "parameter {parameter} requires a value")
            }
            Self::InvalidParameterSource { parameter } => {
                write!(
                    formatter,
                    "parameter {parameter} arrived from an invalid source"
                )
            }
            Self::ZeroRetryAttempts => formatter.write_str("retry attempts must be bounded"),
            Self::UnsafeRetryEffect { effect } => {
                write!(formatter, "retry effect {effect} is not host-mediated")
            }
            Self::InvalidHash { field } => write!(formatter, "{field} must be a BLAKE3 hex hash"),
        }
    }
}

fn parse_workflow_value<T>(
    field_name: crate::validation::FieldName,
    value: &str,
    parser: impl FnOnce(&str) -> Option<T>,
) -> Result<T, RecipeWorkflowError> {
    let normalized = crate::validation::normalized_adapter_field(field_name, value)?;
    parser(normalized.as_str()).ok_or(RecipeWorkflowError::UnsupportedWorkflowValue {
        field: field_name.as_str(),
        value: normalized,
    })
}

fn required_text(
    field_name: crate::validation::FieldName,
    value: &str,
) -> Result<alloc::string::String, RecipeWorkflowError> {
    crate::validation::normalized_adapter_field(field_name, value)
        .map_err(RecipeWorkflowError::from)
}

fn optional_text(
    field_name: crate::validation::FieldName,
    value: Option<&str>,
) -> Result<Option<alloc::string::String>, RecipeWorkflowError> {
    value
        .map(|text| required_text(field_name, text))
        .transpose()
}

fn text_collection(
    field_name: crate::validation::FieldName,
    values: &[&str],
    required: bool,
) -> Result<alloc::vec::Vec<alloc::string::String>, RecipeWorkflowError> {
    if required && values.is_empty() {
        return Err(RecipeWorkflowError::EmptyCollection {
            field: field_name.as_str(),
        });
    }

    let mut normalized = alloc::vec::Vec::with_capacity(values.len());
    for value in values {
        let text = required_text(field_name, value)?;
        if contains_text(normalized.as_slice(), text.as_str()) {
            return Err(RecipeWorkflowError::DuplicateName {
                field: field_name.as_str(),
                value: text,
            });
        }
        normalized.push(text);
    }
    Ok(normalized)
}

fn contains_text(values: &[alloc::string::String], needle: &str) -> bool {
    values.iter().any(|value| value.as_str() == needle)
}
