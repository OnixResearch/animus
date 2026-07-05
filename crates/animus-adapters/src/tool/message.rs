/// Request sent from the harness to a tool component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    name: super::Text,
    input: super::Text,
}

impl Request {
    /// Builds a tool request from a required tool name and optional input text.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the tool name is empty,
    /// either field is too long, or either field contains a NUL character.
    pub fn new(name: &str, input: &str) -> Result<Self, crate::AdapterContractError> {
        // r[impl agent_harness_runtime.boundary_versions.normalized_text]
        let name = crate::validation::normalized_adapter_field(super::NAME_FIELD, name)?;
        crate::validation::validate_optional_adapter_field(super::INPUT_FIELD, input)?;
        Ok(Self {
            name,
            input: super::Text::from(input),
        })
    }

    /// Returns the requested tool name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the tool input text.
    #[must_use]
    pub fn input(&self) -> &str {
        self.input.as_str()
    }

    /// Returns the stable tool request contract identity.
    #[must_use]
    pub const fn contract_identity() -> &'static str {
        super::TOOL_REQUEST_CONTRACT_IDENTITY
    }
}

/// Response returned by a tool component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response {
    output: super::Text,
}

impl Response {
    /// Builds a tool response from optional output text.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the output is too long or
    /// contains a NUL character.
    pub fn new(output: &str) -> Result<Self, crate::AdapterContractError> {
        crate::validation::validate_optional_adapter_field(super::OUTPUT_FIELD, output)?;
        Ok(Self {
            output: super::Text::from(output),
        })
    }

    /// Returns the tool output text.
    #[must_use]
    pub fn output(&self) -> &str {
        self.output.as_str()
    }

    /// Returns the stable tool response contract identity.
    #[must_use]
    pub const fn contract_identity() -> &'static str {
        super::TOOL_RESPONSE_CONTRACT_IDENTITY
    }
}
