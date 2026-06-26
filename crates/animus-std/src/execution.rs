/// Runs shell orchestration with the default deterministic model adapter.
///
/// # Errors
///
/// Returns [`RunError::Model`] if the default adapter rejects its typed request.
pub fn run(
    config: &crate::RuntimeConfig,
) -> Result<RunReport, RunError<animus_adapters::AdapterContractError>> {
    let mut model = SummaryModelAdapter;
    run_with_model(config, &mut model)
}

/// Runs shell orchestration with a caller-provided model adapter.
///
/// # Errors
///
/// Returns [`RunError::Model`] when the supplied model adapter fails.
pub fn run_with_model<A>(
    config: &crate::RuntimeConfig,
    model: &mut A,
) -> Result<RunReport, RunError<A::Error>>
where
    A: animus_adapters::ModelAdapter,
{
    let state = animus_core::AnimusState::new(config.intent().clone());
    let request = animus_adapters::ModelRequest::from_state(&state);
    let response = model
        .call(&request)
        .map_err(|source| RunError::Model { source })?;

    Ok(RunReport {
        summary: state.summary(),
        model_message: String::from(response.message()),
    })
}

/// Default deterministic model adapter used by [`run`].
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SummaryModelAdapter;

impl
    animus_adapters::ComponentAdapter<animus_adapters::ModelRequest, animus_adapters::ModelResponse>
    for SummaryModelAdapter
{
    type Error = animus_adapters::AdapterContractError;

    fn call(
        &mut self,
        request: &animus_adapters::ModelRequest,
    ) -> Result<animus_adapters::ModelResponse, Self::Error> {
        animus_adapters::ModelResponse::new(request.prompt())
    }
}

/// A deterministic report produced by [`run`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunReport {
    summary: String,
    model_message: String,
}

impl RunReport {
    /// Returns the rendered core summary.
    #[must_use]
    pub fn summary(&self) -> &str {
        self.summary.as_str()
    }

    /// Returns the message produced by the model adapter.
    #[must_use]
    pub fn model_message(&self) -> &str {
        self.model_message.as_str()
    }
}

/// Runtime orchestration errors raised after configuration is typed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RunError<AdapterError> {
    /// The selected model adapter failed.
    Model {
        /// Adapter-specific error.
        source: AdapterError,
    },
}

impl<AdapterError> std::fmt::Display for RunError<AdapterError>
where
    AdapterError: std::fmt::Display,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Model { source } => write!(formatter, "model adapter failed: {source}"),
        }
    }
}

impl<AdapterError> std::error::Error for RunError<AdapterError> where
    AdapterError: std::fmt::Debug + std::fmt::Display
{
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_INTENT: &str = "build reliable systems";
    const SUMMARY: &str = "intent: build reliable systems";
    const CUSTOM_MODEL_MESSAGE: &str = "custom adapter response";

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ModelFailure;

    impl std::fmt::Display for ModelFailure {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("model failure")
        }
    }

    struct FixedModelAdapter;

    impl
        animus_adapters::ComponentAdapter<
            animus_adapters::ModelRequest,
            animus_adapters::ModelResponse,
        > for FixedModelAdapter
    {
        type Error = animus_adapters::AdapterContractError;

        fn call(
            &mut self,
            request: &animus_adapters::ModelRequest,
        ) -> Result<animus_adapters::ModelResponse, Self::Error> {
            assert_eq!(request.prompt(), SUMMARY);
            animus_adapters::ModelResponse::new(CUSTOM_MODEL_MESSAGE)
        }
    }

    struct FailingModelAdapter;

    impl
        animus_adapters::ComponentAdapter<
            animus_adapters::ModelRequest,
            animus_adapters::ModelResponse,
        > for FailingModelAdapter
    {
        type Error = ModelFailure;

        fn call(
            &mut self,
            _request: &animus_adapters::ModelRequest,
        ) -> Result<animus_adapters::ModelResponse, Self::Error> {
            Err(ModelFailure)
        }
    }

    #[test]
    fn returns_deterministic_report() {
        let config =
            crate::RuntimeConfig::from_intent(VALID_INTENT).expect("valid intent should parse");
        let outcome = run(&config).expect("default adapter should run");

        assert_eq!(outcome.summary(), SUMMARY);
        assert_eq!(outcome.model_message(), SUMMARY);
    }

    #[test]
    fn accepts_swappable_model_adapter() {
        let config =
            crate::RuntimeConfig::from_intent(VALID_INTENT).expect("valid intent should parse");
        let mut adapter = FixedModelAdapter;
        let outcome = run_with_model(&config, &mut adapter).expect("custom adapter should run");

        assert_eq!(outcome.summary(), SUMMARY);
        assert_eq!(outcome.model_message(), CUSTOM_MODEL_MESSAGE);
    }

    #[test]
    fn reports_model_adapter_failure() {
        let config =
            crate::RuntimeConfig::from_intent(VALID_INTENT).expect("valid intent should parse");
        let mut adapter = FailingModelAdapter;
        let error =
            run_with_model(&config, &mut adapter).expect_err("model failure should surface");

        assert_eq!(
            error,
            RunError::Model {
                source: ModelFailure,
            }
        );
    }
}
