#![allow(unknown_lints, path_segment_repetition)]

use super::Bounds;
use crate::{BudgetExhausted, BudgetKind};

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
        self.consume(self.remaining_candidates, BudgetKind::Candidate, |next| {
            next.remaining_candidates -= 1
        })
    }

    pub fn consume_parallelism(&self) -> Result<Self, BudgetExhausted> {
        self.consume(
            self.remaining_parallelism,
            BudgetKind::Parallelism,
            |next| next.remaining_parallelism -= 1,
        )
    }

    pub fn consume_synthesis_attempt(&self) -> Result<Self, BudgetExhausted> {
        self.consume(
            self.remaining_synthesis_attempts,
            BudgetKind::Synthesis,
            |next| next.remaining_synthesis_attempts -= 1,
        )
    }

    pub fn consume_repair_attempt(&self) -> Result<Self, BudgetExhausted> {
        self.consume(self.remaining_repair_attempts, BudgetKind::Repair, |next| {
            next.remaining_repair_attempts -= 1
        })
    }

    pub fn record_tried_model(&self, label: &str) -> Result<Self, BudgetExhausted> {
        if self
            .tried_models
            .iter()
            .any(|tried| tried.as_str() == label)
        {
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

    fn consume(
        &self,
        remaining: u32,
        kind: BudgetKind,
        update: impl FnOnce(&mut Self),
    ) -> Result<Self, BudgetExhausted> {
        if remaining == 0 {
            return Err(BudgetExhausted::new(kind, remaining));
        }
        let mut next = self.clone();
        update(&mut next);
        Ok(next)
    }
}
