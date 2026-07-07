#![allow(non_trait_imports, missing_docs)]

use crate::{BudgetExhausted, BudgetKind};

const DEADLINE_FIELD: crate::FieldName = crate::FieldName::new("router_timeout_class");
const FALLBACK_FIELD: crate::FieldName = crate::FieldName::new("router_fallback_policy");
pub(super) const ZERO_COUNT: u32 = 0;

/// Timeout/deadline class for bounded host execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeadlineClass {
    /// Short deadline bucket.
    Short,
    /// Standard deadline bucket.
    Standard,
    /// Extended deadline bucket.
    Extended,
}

impl DeadlineClass {
    /// Parses a deadline class.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] for unsupported labels.
    pub fn parse(value: &str) -> Result<Self, crate::AdapterContractError> {
        crate::parse_value(DEADLINE_FIELD, value, |normalized| match normalized {
            crate::policy::DEADLINE_SHORT => Some(Self::Short),
            crate::policy::DEADLINE_STANDARD => Some(Self::Standard),
            crate::policy::DEADLINE_EXTENDED => Some(Self::Extended),
            _ => None,
        })
    }

    /// Returns stable deadline label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Short => crate::policy::DEADLINE_SHORT,
            Self::Standard => crate::policy::DEADLINE_STANDARD,
            Self::Extended => crate::policy::DEADLINE_EXTENDED,
        }
    }
}

/// Fallback policy for collaborative route failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FallbackPolicy {
    /// Deny fallback.
    Deny,
    /// Return best valid evidence when policy permits.
    BestValidEvidence,
    /// Fall back to a safe single model.
    SafeSingle,
}

impl FallbackPolicy {
    /// Parses fallback policy.
    ///
    /// # Errors
    ///
    /// Returns [`crate::AdapterContractError`] for unsupported labels.
    pub fn parse(value: &str) -> Result<Self, crate::AdapterContractError> {
        crate::parse_value(FALLBACK_FIELD, value, |normalized| match normalized {
            crate::policy::FALLBACK_DENY => Some(Self::Deny),
            crate::policy::FALLBACK_BEST_VALID_EVIDENCE => Some(Self::BestValidEvidence),
            crate::policy::FALLBACK_SAFE_SINGLE => Some(Self::SafeSingle),
            _ => None,
        })
    }

    /// Returns stable fallback label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deny => crate::policy::FALLBACK_DENY,
            Self::BestValidEvidence => crate::policy::FALLBACK_BEST_VALID_EVIDENCE,
            Self::SafeSingle => crate::policy::FALLBACK_SAFE_SINGLE,
        }
    }
}

/// Hard execution bounds validated before host execution begins.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bounds {
    max_candidates: u32,
    max_parallelism: u32,
    min_success_quorum: u32,
    deadline: DeadlineClass,
    max_synthesis_attempts: u32,
    max_repair_attempts: u32,
    fallback_policy: FallbackPolicy,
}

impl Bounds {
    /// Builds validated execution bounds.
    ///
    /// # Errors
    ///
    /// Returns [`crate::ValidationError`] for unbounded fan-out, impossible quorum,
    /// missing deadline, unlimited repair, or disallowed fallback shapes.
    pub fn new(draft: BoundsDraft<'_>) -> Result<Self, crate::ValidationError> {
        if draft.max_candidates == ZERO_COUNT || draft.max_parallelism == ZERO_COUNT {
            return Err(crate::ValidationError::UnboundedFanOut);
        }
        if draft.min_success_quorum == ZERO_COUNT {
            return Err(crate::ValidationError::ZeroQuorum);
        }
        if draft.min_success_quorum > draft.max_candidates {
            return Err(crate::ValidationError::ImpossibleQuorum);
        }
        let deadline =
            DeadlineClass::parse(draft.timeout_class).map_err(crate::ValidationError::Contract)?;
        let max_repair_attempts = draft
            .max_repair_attempts
            .ok_or(crate::ValidationError::UnlimitedRepair)?;

        Ok(Self {
            max_candidates: draft.max_candidates,
            max_parallelism: draft.max_parallelism,
            min_success_quorum: draft.min_success_quorum,
            deadline,
            max_synthesis_attempts: draft.max_synthesis_attempts,
            max_repair_attempts,
            fallback_policy: FallbackPolicy::parse(draft.fallback_policy)
                .map_err(crate::ValidationError::Contract)?,
        })
    }

    /// Returns max candidates.
    #[must_use]
    pub const fn max_candidates(&self) -> u32 {
        self.max_candidates
    }

    /// Returns max parallelism.
    #[must_use]
    pub const fn max_parallelism(&self) -> u32 {
        self.max_parallelism
    }

    /// Returns min-success quorum.
    #[must_use]
    pub const fn min_success_quorum(&self) -> u32 {
        self.min_success_quorum
    }

    /// Returns deadline class.
    #[must_use]
    pub const fn timeout_class(&self) -> DeadlineClass {
        self.deadline
    }

    /// Returns max synthesis attempts.
    #[must_use]
    pub const fn max_synthesis_attempts(&self) -> u32 {
        self.max_synthesis_attempts
    }

    /// Returns max repair attempts.
    #[must_use]
    pub const fn max_repair_attempts(&self) -> u32 {
        self.max_repair_attempts
    }

    /// Returns fallback policy.
    #[must_use]
    pub const fn fallback_policy(&self) -> FallbackPolicy {
        self.fallback_policy
    }
}

/// Borrowed fields used to validate execution bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundsDraft<'a> {
    /// Hard candidate count cap.
    pub max_candidates: u32,
    /// Hard parallelism cap.
    pub max_parallelism: u32,
    /// Minimum successful candidates required.
    pub min_success_quorum: u32,
    /// Timeout class label.
    pub timeout_class: &'a str,
    /// Hard synthesis attempt cap.
    pub max_synthesis_attempts: u32,
    /// Bounded repair attempt cap; `None` is rejected as unlimited repair.
    pub max_repair_attempts: Option<u32>,
    /// Fallback policy label.
    pub fallback_policy: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BudgetLedger {
    candidate_limit: u32,
    remaining_candidates: u32,
    remaining_parallelism: u32,
    remaining_synthesis_attempts: u32,
    remaining_repair_attempts: u32,
    tried_models: alloc::vec::Vec<alloc::string::String>,
}

impl From<&Bounds> for BudgetLedger {
    fn from(bounds: &Bounds) -> Self {
        Self {
            candidate_limit: bounds.max_candidates(),
            remaining_candidates: bounds.max_candidates(),
            remaining_parallelism: bounds.max_parallelism(),
            remaining_synthesis_attempts: bounds.max_synthesis_attempts(),
            remaining_repair_attempts: bounds.max_repair_attempts(),
            tried_models: alloc::vec::Vec::new(),
        }
    }
}

impl BudgetLedger {
    pub const fn candidate_limit(&self) -> u32 {
        self.candidate_limit
    }

    pub const fn remaining_candidates(&self) -> u32 {
        self.remaining_candidates
    }

    pub const fn remaining_parallelism(&self) -> u32 {
        self.remaining_parallelism
    }

    pub const fn remaining_synthesis_attempts(&self) -> u32 {
        self.remaining_synthesis_attempts
    }

    pub const fn remaining_repair_attempts(&self) -> u32 {
        self.remaining_repair_attempts
    }

    pub fn tried_models(&self) -> &[alloc::string::String] {
        self.tried_models.as_slice()
    }

    pub fn consume_candidate(&self) -> Result<Self, BudgetExhausted> {
        self.consume(self.remaining_candidates, BudgetKind::Candidate, |next| next.remaining_candidates -= 1)
    }

    pub fn consume_parallelism(&self) -> Result<Self, BudgetExhausted> {
        self.consume(self.remaining_parallelism, BudgetKind::Parallelism, |next| next.remaining_parallelism -= 1)
    }

    pub fn consume_synthesis_attempt(&self) -> Result<Self, BudgetExhausted> {
        self.consume(self.remaining_synthesis_attempts, BudgetKind::Synthesis, |next| next.remaining_synthesis_attempts -= 1)
    }

    pub fn consume_repair_attempt(&self) -> Result<Self, BudgetExhausted> {
        self.consume(self.remaining_repair_attempts, BudgetKind::Repair, |next| next.remaining_repair_attempts -= 1)
    }

    pub fn record_tried_model(&self, label: &str) -> Result<Self, BudgetExhausted> {
        if self.tried_models.iter().any(|tried| tried.as_str() == label) {
            return Ok(self.clone());
        }
        if self.tried_models.len() as u32 >= self.candidate_limit {
            return Err(BudgetExhausted::new(
                BudgetKind::Candidate,
                self.remaining_candidates,
            ));
        }
        let mut next = self.clone();
        next.tried_models.push(alloc::string::String::from(label));
        Ok(next)
    }

    fn consume(&self, remaining: u32, kind: BudgetKind, update: impl FnOnce(&mut Self)) -> Result<Self, BudgetExhausted> {
        if remaining == 0 {
            return Err(BudgetExhausted::new(kind, remaining));
        }
        let mut next = self.clone();
        update(&mut next);
        Ok(next)
    }
}

#[cfg(test)]
pub(crate) fn test_bounds(fixture: crate::test_support::BoundsFixture) -> Bounds {
    Bounds {
        max_candidates: fixture.max_candidates,
        max_parallelism: fixture.max_parallelism,
        min_success_quorum: fixture.min_success_quorum,
        deadline: fixture.timeout_class,
        max_synthesis_attempts: fixture.max_synthesis_attempts,
        max_repair_attempts: fixture.max_repair_attempts,
        fallback_policy: fixture.fallback_policy,
    }
}
