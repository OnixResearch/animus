//! Named adapter bounds.

use alloc::format;
use alloc::string::String;

use crate::error::WeaveAdapterError;

/// Domain separator for policy identities.
const POLICY_DOMAIN: &str = "animus_adapter_weave_policy_v1";

/// Named bounds for one weave adapter instance.
///
/// Every bound must be positive. The adapter validates the policy once at
/// construction and revalidates every request against it before any weave
/// mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeaveAdapterPolicy {
    node_limit: usize,
    content_bytes: usize,
    parent_fan_in: usize,
}

impl WeaveAdapterPolicy {
    /// Creates a policy after validating that every bound is positive.
    ///
    /// # Errors
    ///
    /// Returns [`WeaveAdapterError::InvalidPolicy`] when any bound is zero.
    pub fn new(
        node_limit: usize,
        content_bytes: usize,
        parent_fan_in: usize,
    ) -> Result<Self, WeaveAdapterError> {
        let policy = Self {
            node_limit,
            content_bytes,
            parent_fan_in,
        };
        policy.validate()?;
        Ok(policy)
    }

    /// Validates that every bound is positive.
    ///
    /// # Errors
    ///
    /// Returns [`WeaveAdapterError::InvalidPolicy`] naming the first zero bound.
    pub fn validate(&self) -> Result<(), WeaveAdapterError> {
        if self.node_limit == 0 {
            return Err(WeaveAdapterError::InvalidPolicy {
                field: "node_limit",
            });
        }
        if self.content_bytes == 0 {
            return Err(WeaveAdapterError::InvalidPolicy {
                field: "content_bytes",
            });
        }
        if self.parent_fan_in == 0 {
            return Err(WeaveAdapterError::InvalidPolicy {
                field: "parent_fan_in",
            });
        }
        Ok(())
    }

    /// Returns the maximum number of nodes the weave may hold.
    #[must_use]
    pub fn node_limit(&self) -> usize {
        self.node_limit
    }

    /// Returns the maximum node content size in bytes.
    #[must_use]
    pub fn content_byte_limit(&self) -> usize {
        self.content_bytes
    }

    /// Returns the maximum number of parents one node may have.
    #[must_use]
    pub fn parent_fan_in_limit(&self) -> usize {
        self.parent_fan_in
    }

    /// Returns the domain-separated BLAKE3 identity of the policy.
    #[must_use]
    pub fn identity(&self) -> String {
        let canonical = format!(
            "{POLICY_DOMAIN}|node_limit={}|content_bytes={}|parent_fan_in={}",
            self.node_limit, self.content_bytes, self.parent_fan_in
        );
        format!("{}", blake3::hash(canonical.as_bytes()).to_hex())
    }
}
