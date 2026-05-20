use crate::calc::TreasuryCalculator;
use crate::model::TreasuryRecord;
use crate::policy::TreasuryPolicy;
use crate::rules;
use crate::score::TreasuryScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct TreasuryPipeline {
    pub policy: TreasuryPolicy,
    pub calc: TreasuryCalculator,
    pub scorer: TreasuryScorer,
}

impl Default for TreasuryPipeline {
    fn default() -> Self {
        Self {
            policy: TreasuryPolicy::default(),
            calc: TreasuryCalculator::default(),
            scorer: TreasuryScorer::tuned(),
        }
    }
}

impl TreasuryPipeline {
    pub fn run_precheck(&self, rec: &mut TreasuryRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 901 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "treasury: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &TreasuryRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(
        &self,
        items: &mut [TreasuryRecord],
        weekday: bool,
    ) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = TreasuryScorer::combine(&scores);
        if combined > 951 {
            return Err(KernelError::Forbidden(
                "treasury: batch risk too high".into(),
            ));
        }
        Ok(scores)
    }
}
