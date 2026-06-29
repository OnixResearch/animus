/// Adapter component kind used in runtime events.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterComponent {
    /// Model adapter component.
    Model,
    /// Tool adapter component.
    Tool,
}

/// Ordered runtime event emitted by the agent loop.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeEvent {
    /// The loop started for a validated runtime input summary.
    LoopStarted {
        /// Deterministic input summary made available to the loop fabric.
        summary: String,
    },
    /// A validated model request is about to cross the model adapter boundary.
    ModelRequested {
        /// Validated model request.
        request: animus_adapters::ModelRequest,
    },
    /// The model adapter returned a validated response.
    ModelResponded {
        /// Validated model response.
        response: animus_adapters::ModelResponse,
    },
    /// A validated tool request is about to cross the tool adapter boundary.
    ToolRequested {
        /// Validated tool request.
        request: animus_adapters::ToolRequest,
    },
    /// The tool adapter returned a validated response.
    ToolResponded {
        /// Validated tool response.
        response: animus_adapters::ToolResponse,
    },
    /// The loop fabric selected a final response.
    FinalResponse {
        /// Validated final model response.
        response: animus_adapters::ModelResponse,
    },
    /// The loop fabric selected a stop decision without a final response.
    Stopped,
    /// The configured maximum-turn limit was reached before completion.
    MaxTurnsExceeded {
        /// Configured maximum number of fabric turns.
        max_turns: usize,
    },
    /// An adapter returned an error and the runtime halted.
    AdapterError {
        /// Adapter component that failed.
        component: AdapterComponent,
        /// Human-readable adapter error message.
        message: String,
    },
    /// The loop fabric returned an error and the runtime halted.
    FabricError {
        /// Human-readable fabric error message.
        message: String,
    },
    /// Steel policy denied activation before Steel evaluation.
    SteelPolicyDenied {
        /// Safe selected profile label.
        profile: String,
        /// BLAKE3 hash of observed script source.
        script_hash: String,
        /// BLAKE3 hash of safe receipt fields.
        receipt_hash: String,
        /// Typed denial reason label.
        reason: String,
    },
}
