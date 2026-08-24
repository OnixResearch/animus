//! Bounded synchronized store for advisory tool-result reuse.
//!
//! The pure `animus-core` contract decides retention. This shell owns locking,
//! generation counters, stored values, and release execution. Adapter work runs
//! only after a miss and outside the synchronization guard.

mod mutation;
mod operation;
mod storage;

/// User-supplied configuration with no implicit capacity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Config {
    capacity: Option<u32>,
    low_watermark: u32,
    high_watermark: u32,
    promotion_threshold_percent: u8,
}

impl Config {
    /// Builds a configuration from explicit values.
    #[must_use]
    pub const fn new(
        capacity: Option<u32>,
        low_watermark: u32,
        high_watermark: u32,
        promotion_threshold_percent: u8,
    ) -> Self {
        Self {
            capacity,
            low_watermark,
            high_watermark,
            promotion_threshold_percent,
        }
    }
}

/// Bounded synchronized store for shared tool-result values.
pub struct Store<Value> {
    policy: animus_core::retention::Policy,
    state: std::sync::Arc<std::sync::Mutex<storage::State<Value>>>,
}

/// Result of an insertion attempt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InsertOutcome {
    /// Explicit zero capacity retained nothing.
    NotRetained,
    /// A previous value for the same key was replaced.
    Replaced,
    /// A new key was inserted, with zero or more watermark evictions.
    Inserted {
        /// Digests removed by capacity or watermark policy.
        evicted_keys: std::vec::Vec<animus_core::retention::Digest>,
    },
}

/// Source of a value returned by [`Store::get_or_try_insert_with`].
#[derive(Debug)]
pub enum Access<Value> {
    /// Existing advisory value.
    Hit(std::sync::Arc<Value>),
    /// Adapter-produced value.
    Miss(std::sync::Arc<Value>),
}

impl<Value> Access<Value> {
    /// Returns the shared value regardless of its source.
    #[must_use]
    pub fn value(&self) -> &std::sync::Arc<Value> {
        match self {
            Self::Hit(value) | Self::Miss(value) => value,
        }
    }

    /// Returns true only for an existing entry.
    #[must_use]
    pub const fn is_hit(&self) -> bool {
        matches!(self, Self::Hit(_))
    }
}

/// Failure from either the store shell or the unchanged adapter miss path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LoadError<AdapterError> {
    /// Store synchronization or state failure.
    Store(StoreError),
    /// Existing adapter failure.
    Adapter(AdapterError),
}

/// Bounded store construction or execution failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StoreError {
    /// No explicit capacity was supplied.
    MissingCapacity,
    /// Pure policy or supplied-state rejection.
    Decision(animus_core::retention::DecisionError),
    /// A holder panicked while synchronized.
    LockPoisoned,
    /// Monotonic generation reached its representable limit.
    GenerationOverflow,
    /// Shell state contradicted a core decision.
    StateInvariant,
}

impl From<animus_core::retention::DecisionError> for StoreError {
    fn from(error: animus_core::retention::DecisionError) -> Self {
        Self::Decision(error)
    }
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingCapacity => formatter.write_str("tool-result cache capacity is required"),
            Self::Decision(error) => write!(formatter, "tool-result cache policy failed: {error}"),
            Self::LockPoisoned => formatter.write_str("tool-result cache lock is poisoned"),
            Self::GenerationOverflow => {
                formatter.write_str("tool-result cache generation overflowed")
            }
            Self::StateInvariant => formatter.write_str("tool-result cache state is inconsistent"),
        }
    }
}

impl std::error::Error for StoreError {}

#[cfg(test)]
mod tests;
