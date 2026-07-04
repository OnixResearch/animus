/// Tool-contract validation and planning errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractError {
    /// A common adapter field check failed.
    Adapter(crate::AdapterContractError),
    /// A required optional draft field was absent.
    MissingField {
        /// Boundary field name.
        field: &'static str,
    },
    /// A collection exceeded the tool contract bound.
    TooManyItems {
        /// Boundary field name.
        field: &'static str,
        /// Maximum accepted item count.
        max_items: usize,
        /// Actual item count.
        actual_items: usize,
    },
    /// A prepared argument collided with a previous argument.
    DuplicateArgument {
        /// Duplicate argument name.
        name: super::Text,
    },
    /// A prepared argument is not part of the current strict contract.
    UnknownArgument {
        /// Unknown argument name.
        name: super::Text,
    },
    /// A safe output reference looked like a raw path, URL, or host locator.
    UnsafeReference {
        /// Rejected reference text.
        reference: super::Text,
    },
    /// The visible-output budget was outside the supported range.
    InvalidVisibleBudget {
        /// Smallest accepted budget.
        min_chars: u32,
        /// Largest accepted budget.
        max_chars: u32,
        /// Actual requested budget.
        actual_chars: u32,
    },
}

impl From<crate::AdapterContractError> for ContractError {
    fn from(error: crate::AdapterContractError) -> Self {
        Self::Adapter(error)
    }
}
