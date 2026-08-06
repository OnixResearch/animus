//! Typed adapter errors.

use core::fmt;

/// Errors returned by the weave adapter.
///
/// Every variant leaves the weave unchanged. Variants carry only bounded
/// numeric metadata, never node text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeaveAdapterError {
    /// A policy bound was zero at construction.
    InvalidPolicy {
        /// Name of the invalid bound field.
        field: &'static str,
    },
    /// Node text exceeded the configured byte bound.
    ContentTooLarge {
        /// Configured byte limit.
        limit: usize,
        /// Actual content size in bytes.
        actual: usize,
    },
    /// The weave already holds the configured maximum number of nodes.
    NodeLimitReached {
        /// Configured node limit.
        limit: usize,
    },
    /// A request named more parents than the configured fan-in bound.
    TooManyParents {
        /// Configured fan-in limit.
        limit: usize,
        /// Actual number of parents in the request.
        actual: usize,
    },
    /// A request named a node identifier that already exists.
    NodeExists {
        /// The conflicting node identifier.
        id: u128,
    },
    /// A request named a node identifier that does not exist.
    NodeMissing {
        /// The missing node identifier.
        id: u128,
    },
    /// A request named a parent identifier that does not exist.
    ParentMissing {
        /// The missing parent identifier.
        id: u128,
    },
    /// The upstream weave rejected a split.
    SplitRejected {
        /// The target node identifier.
        id: u128,
    },
    /// The upstream weave rejected a merge with the parent.
    MergeRejected {
        /// The target node identifier.
        id: u128,
    },
    /// The upstream weave rejected a move, for example because it would create a cycle.
    MoveRejected {
        /// The target node identifier.
        id: u128,
    },
    /// The upstream weave rejected an operation for an unspecified consistency reason.
    OperationRejected {
        /// The target node identifier.
        id: u128,
    },
    /// A consistency check found malformed weave state.
    InconsistentWeave,
}

impl fmt::Display for WeaveAdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPolicy { field } => write!(formatter, "invalid policy bound: {field}"),
            Self::ContentTooLarge { limit, actual } => {
                write!(formatter, "content bytes {actual} exceed limit {limit}")
            }
            Self::NodeLimitReached { limit } => write!(formatter, "node limit {limit} reached"),
            Self::TooManyParents { limit, actual } => {
                write!(formatter, "parent count {actual} exceeds limit {limit}")
            }
            Self::NodeExists { id } => write!(formatter, "node {id} already exists"),
            Self::NodeMissing { id } => write!(formatter, "node {id} does not exist"),
            Self::ParentMissing { id } => write!(formatter, "parent {id} does not exist"),
            Self::SplitRejected { id } => write!(formatter, "split rejected for node {id}"),
            Self::MergeRejected { id } => write!(formatter, "merge rejected for node {id}"),
            Self::MoveRejected { id } => write!(formatter, "move rejected for node {id}"),
            Self::OperationRejected { id } => {
                write!(formatter, "operation rejected for node {id}")
            }
            Self::InconsistentWeave => write!(formatter, "weave state is inconsistent"),
        }
    }
}

impl core::error::Error for WeaveAdapterError {}
