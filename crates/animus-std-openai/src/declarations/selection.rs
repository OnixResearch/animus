/// Exact tool-selection policy for one provider request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiToolChoice {
    kind: ChoiceKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ChoiceKind {
    Auto,
    Required,
    Named(String),
}

impl OpenAiToolChoice {
    /// Lets the provider select no tool or one declared tool.
    #[must_use]
    pub const fn auto() -> Self {
        Self {
            kind: ChoiceKind::Auto,
        }
    }

    /// Requires the provider to select one declared tool.
    #[must_use]
    pub const fn required() -> Self {
        Self {
            kind: ChoiceKind::Required,
        }
    }

    /// Requires one exact declared tool.
    ///
    /// # Errors
    /// Returns [`crate::HostError`] when the tool name is invalid.
    pub fn named(name: &str) -> Result<Self, crate::HostError> {
        super::validate_label(name)?;
        Ok(Self {
            kind: ChoiceKind::Named(String::from(name)),
        })
    }

    pub(super) fn project(
        &self,
        request: &animus_std::OpenAiResponsesDispatchRequest,
    ) -> Result<serde_json::Value, crate::HostError> {
        match &self.kind {
            ChoiceKind::Auto => Ok(serde_json::Value::String(String::from("auto"))),
            ChoiceKind::Required => Ok(serde_json::Value::String(String::from("required"))),
            ChoiceKind::Named(name) => {
                if !request.tools().iter().any(|tool| tool.name() == name) {
                    return super::unsupported("tool_choice");
                }
                Ok(serde_json::json!({"type": "function", "name": name}))
            }
        }
    }
}

/// Explicit serial tool policy for the reviewed provider cohort.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiToolPolicy {
    choice: OpenAiToolChoice,
    parallel_tool_calls: bool,
}

impl OpenAiToolPolicy {
    /// Builds the reviewed serial tool policy.
    #[must_use]
    pub const fn serial(choice: OpenAiToolChoice) -> Self {
        Self {
            choice,
            parallel_tool_calls: false,
        }
    }

    /// Returns the exact tool choice.
    #[must_use]
    pub const fn choice(&self) -> &OpenAiToolChoice {
        &self.choice
    }

    /// Returns whether parallel function calls are allowed.
    #[must_use]
    pub const fn parallel_tool_calls(&self) -> bool {
        self.parallel_tool_calls
    }
}
