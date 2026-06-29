const ID_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("ledger_entry_id");
const TEXT_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("ledger_text");
const LABEL_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("ledger_label");

/// Validated neutral ledger entry identifier.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EntryId {
    value: alloc::string::String,
}

impl EntryId {
    /// Builds a validated entry identifier.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the identifier is empty,
    /// overlong, or contains a NUL character.
    pub fn new(value: &str) -> Result<Self, crate::AdapterContractError> {
        Ok(Self {
            value: crate::validation::normalized_adapter_field(ID_FIELD, value)?,
        })
    }

    /// Returns the validated identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

/// Kind of neutral ledger entry.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EntryKind {
    /// User-authored model-visible message.
    UserMessage,
    /// Assistant-authored model-visible message.
    AssistantMessage,
    /// Tool result made visible to the model.
    ToolResult,
    /// Bounded summary made visible as context.
    Summary,
    /// Usage observation retained as safe metadata.
    UsageObservation,
    /// Safe receipt retained as metadata.
    SafeReceipt,
    /// Unsupported entry class that must fail closed.
    Unsupported,
    /// Adapter-owned payload marker that must not be replayed by the neutral core.
    AdapterOwned,
}

impl EntryKind {
    /// Returns a stable kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserMessage => "user_message",
            Self::AssistantMessage => "assistant_message",
            Self::ToolResult => "tool_result",
            Self::Summary => "summary",
            Self::UsageObservation => "usage_observation",
            Self::SafeReceipt => "safe_receipt",
            Self::Unsupported => "unsupported",
            Self::AdapterOwned => "adapter_owned",
        }
    }
}

/// Neutral ledger entry owned by Animus adapter boundaries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Entry {
    /// User-authored model-visible message.
    UserMessage {
        /// Entry identifier.
        id: EntryId,
        /// Model-visible content.
        content: alloc::string::String,
    },
    /// Assistant-authored model-visible message.
    AssistantMessage {
        /// Entry identifier.
        id: EntryId,
        /// Model-visible content.
        content: alloc::string::String,
    },
    /// Tool result made visible to the model.
    ToolResult {
        /// Entry identifier.
        id: EntryId,
        /// Safe tool label.
        tool_label: alloc::string::String,
        /// Model-visible tool content.
        content: alloc::string::String,
    },
    /// Bounded summary made visible as context.
    Summary {
        /// Entry identifier.
        id: EntryId,
        /// Summary content.
        content: alloc::string::String,
    },
    /// Usage observation retained as metadata counts.
    UsageObservation {
        /// Entry identifier.
        id: EntryId,
        /// Observed input token count.
        input_tokens: u32,
        /// Observed output token count.
        output_tokens: u32,
    },
    /// Safe receipt retained as replay metadata.
    SafeReceipt {
        /// Entry identifier.
        id: EntryId,
        /// Safe receipt payload.
        receipt: crate::SafeReceipt,
    },
    /// Unsupported entry that must stop replay.
    Unsupported {
        /// Entry identifier.
        id: EntryId,
        /// Safe unsupported kind label.
        kind_label: alloc::string::String,
    },
    /// Adapter-owned payload marker that must stop neutral replay.
    AdapterOwned {
        /// Entry identifier.
        id: EntryId,
        /// Safe adapter-owned kind label.
        kind_label: alloc::string::String,
    },
}

impl Entry {
    /// Builds a user message entry.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when an entry field is invalid.
    pub fn user(id: &str, content: &str) -> Result<Self, crate::AdapterContractError> {
        Ok(Self::UserMessage {
            id: EntryId::new(id)?,
            content: normalized_text(content)?,
        })
    }

    /// Builds an assistant message entry.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when an entry field is invalid.
    pub fn assistant(id: &str, content: &str) -> Result<Self, crate::AdapterContractError> {
        Ok(Self::AssistantMessage {
            id: EntryId::new(id)?,
            content: normalized_text(content)?,
        })
    }

    /// Builds a tool result entry.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when an entry field is invalid.
    pub fn tool(
        id: &str,
        tool_label: &str,
        content: &str,
    ) -> Result<Self, crate::AdapterContractError> {
        Ok(Self::ToolResult {
            id: EntryId::new(id)?,
            tool_label: normalized_label(tool_label)?,
            content: normalized_text(content)?,
        })
    }

    /// Builds a summary entry.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when an entry field is invalid.
    pub fn summary(id: &str, content: &str) -> Result<Self, crate::AdapterContractError> {
        Ok(Self::Summary {
            id: EntryId::new(id)?,
            content: normalized_text(content)?,
        })
    }

    /// Builds a usage observation entry.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the identifier is invalid.
    pub fn usage(
        id: &str,
        input_tokens: u32,
        output_tokens: u32,
    ) -> Result<Self, crate::AdapterContractError> {
        Ok(Self::UsageObservation {
            id: EntryId::new(id)?,
            input_tokens,
            output_tokens,
        })
    }

    /// Builds a safe receipt entry.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when the identifier is invalid.
    pub fn safe_receipt(
        id: &str,
        receipt: crate::SafeReceipt,
    ) -> Result<Self, crate::AdapterContractError> {
        Ok(Self::SafeReceipt {
            id: EntryId::new(id)?,
            receipt,
        })
    }

    /// Builds an unsupported entry marker without retaining raw payload.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when an entry field is invalid.
    pub fn unsupported(id: &str, kind_label: &str) -> Result<Self, crate::AdapterContractError> {
        Ok(Self::Unsupported {
            id: EntryId::new(id)?,
            kind_label: normalized_label(kind_label)?,
        })
    }

    /// Builds an adapter-owned marker without retaining raw payload.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] when an entry field is invalid.
    pub fn adapter_owned(id: &str, kind_label: &str) -> Result<Self, crate::AdapterContractError> {
        Ok(Self::AdapterOwned {
            id: EntryId::new(id)?,
            kind_label: normalized_label(kind_label)?,
        })
    }

    /// Returns the entry identifier.
    #[must_use]
    pub const fn id(&self) -> &EntryId {
        match self {
            Self::UserMessage { id, .. }
            | Self::AssistantMessage { id, .. }
            | Self::ToolResult { id, .. }
            | Self::Summary { id, .. }
            | Self::UsageObservation { id, .. }
            | Self::SafeReceipt { id, .. }
            | Self::Unsupported { id, .. }
            | Self::AdapterOwned { id, .. } => id,
        }
    }

    /// Returns the entry kind.
    #[must_use]
    pub const fn kind(&self) -> EntryKind {
        match self {
            Self::UserMessage { .. } => EntryKind::UserMessage,
            Self::AssistantMessage { .. } => EntryKind::AssistantMessage,
            Self::ToolResult { .. } => EntryKind::ToolResult,
            Self::Summary { .. } => EntryKind::Summary,
            Self::UsageObservation { .. } => EntryKind::UsageObservation,
            Self::SafeReceipt { .. } => EntryKind::SafeReceipt,
            Self::Unsupported { .. } => EntryKind::Unsupported,
            Self::AdapterOwned { .. } => EntryKind::AdapterOwned,
        }
    }
}

fn normalized_text(value: &str) -> Result<alloc::string::String, crate::AdapterContractError> {
    crate::validation::normalized_adapter_field(TEXT_FIELD, value)
}

fn normalized_label(value: &str) -> Result<alloc::string::String, crate::AdapterContractError> {
    crate::validation::normalized_adapter_field(LABEL_FIELD, value)
}
