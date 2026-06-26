const NAME_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("tool_name");
const INPUT_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("tool_input");
const OUTPUT_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("tool_output");

/// Request sent from the harness to a tool component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    name: alloc::string::String,
    input: alloc::string::String,
}

impl Request {
    /// Builds a tool request from a required tool name and optional input text.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the tool name is empty,
    /// either field is too long, or either field contains a NUL character.
    pub fn new(name: &str, input: &str) -> Result<Self, crate::AdapterContractError> {
        crate::validation::validate_required_adapter_field(NAME_FIELD, name)?;
        crate::validation::validate_optional_adapter_field(INPUT_FIELD, input)?;
        Ok(Self {
            name: alloc::string::String::from(name),
            input: alloc::string::String::from(input),
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
}

/// Response returned by a tool component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response {
    output: alloc::string::String,
}

impl Response {
    /// Builds a tool response from optional output text.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the output is too long or
    /// contains a NUL character.
    pub fn new(output: &str) -> Result<Self, crate::AdapterContractError> {
        crate::validation::validate_optional_adapter_field(OUTPUT_FIELD, output)?;
        Ok(Self {
            output: alloc::string::String::from(output),
        })
    }

    /// Returns the tool output text.
    #[must_use]
    pub fn output(&self) -> &str {
        self.output.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOOL_NAME: &str = "search";
    const TOOL_INPUT: &str = "query";
    const TOOL_OUTPUT: &str = "result";
    const BLANK_TEXT: &str = "   ";

    #[test]
    fn request_accepts_empty_input() {
        let request = Request::new(TOOL_NAME, "").expect("empty input should be allowed");

        assert_eq!(request.name(), TOOL_NAME);
        assert_eq!(request.input(), "");
    }

    #[test]
    fn request_accepts_valid_input() {
        let request = Request::new(TOOL_NAME, TOOL_INPUT).expect("tool request should parse");

        assert_eq!(request.name(), TOOL_NAME);
        assert_eq!(request.input(), TOOL_INPUT);
    }

    #[test]
    fn request_rejects_missing_name() {
        let error = Request::new(BLANK_TEXT, TOOL_INPUT).expect_err("blank tool name should fail");

        assert_eq!(
            error,
            crate::AdapterContractError::EmptyField {
                field: NAME_FIELD.as_str(),
            }
        );
    }

    #[test]
    fn response_accepts_empty_output() {
        let response = Response::new("").expect("empty output should be allowed");

        assert_eq!(response.output(), "");
    }

    #[test]
    fn response_accepts_valid_output() {
        let response = Response::new(TOOL_OUTPUT).expect("tool output should parse");

        assert_eq!(response.output(), TOOL_OUTPUT);
    }
}
