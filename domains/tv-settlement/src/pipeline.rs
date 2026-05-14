use crate::calc::SettlementCalculator;
use crate::model::SettlementRecord;
use crate::policy::SettlementPolicy;
use crate::rules;
use crate::score::SettlementScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct SettlementPipeline {
    pub policy: SettlementPolicy,
    pub calc: SettlementCalculator,
    pub scorer: SettlementScorer,
}

impl Default for SettlementPipeline {
    fn default() -> Self {
        Self {
            policy: SettlementPolicy::default(),
            calc: SettlementCalculator::default(),
            scorer: SettlementScorer::tuned(),
        }
    }
}

impl SettlementPipeline {
    pub fn run_precheck(&self, rec: &mut SettlementRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 937 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "settlement: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &SettlementRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(
        &self,
        items: &mut [SettlementRecord],
        weekday: bool,
    ) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = SettlementScorer::combine(&scores);
        if combined > 887 {
            return Err(KernelError::Forbidden(
                "settlement: batch risk too high".into(),
            ));
        }
        Ok(scores)
    }
}
