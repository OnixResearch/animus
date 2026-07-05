mod basic;
mod data;
mod flow;

pub use basic::{ApiKind, EndpointIdentity, InputModality, TokenCost};
pub use data::{ModelCapabilities, ModelCapabilitiesDraft};
pub use flow::{
    ModelStreamingCapabilities, ModelStreamingRequest, ResolvedStreamingModes, StreamFeature,
    StreamRequestMode, StreamSupport, UnsupportedStreamPolicy,
};

const MAX_ENDPOINT_BYTES: usize = 256;
const MAX_MODALITY_COUNT: usize = 4;
const MAX_THINKING_LEVEL_COUNT: usize = 4;
const DEFAULT_CLAMP_FALLBACK: super::ThinkingLevel = super::ThinkingLevel::None;

pub(super) struct CollectionBounds {
    field: &'static str,
    actual_len: usize,
    max_len: usize,
}

impl CollectionBounds {
    pub(super) const fn new(field: &'static str, actual_len: usize, max_len: usize) -> Self {
        Self {
            field,
            actual_len,
            max_len,
        }
    }
}

pub(super) fn non_zero_budget(
    field: &'static str,
    budget: u32,
) -> Result<core::num::NonZeroU32, super::RegistryError> {
    core::num::NonZeroU32::new(budget).ok_or(super::RegistryError::InvalidBudget {
        field,
        reason: super::INVALID_BUDGET_REASON,
    })
}

pub(super) fn validate_collection(bounds: &CollectionBounds) -> Result<(), super::RegistryError> {
    if bounds.actual_len == 0 {
        return Err(super::RegistryError::EmptyCollection {
            field: bounds.field,
            reason: super::EMPTY_COLLECTION_REASON,
        });
    }

    if bounds.actual_len > bounds.max_len {
        return Err(super::RegistryError::CollectionLimit {
            field: bounds.field,
            reason: super::COLLECTION_LIMIT_REASON,
        });
    }

    Ok(())
}
