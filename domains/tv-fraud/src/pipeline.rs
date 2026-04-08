use crate::calc::FraudCalculator;
use crate::model::FraudRecord;
use crate::policy::FraudPolicy;
use crate::rules;
use crate::score::FraudScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct FraudPipeline {
    pub policy: FraudPolicy,
    pub calc: FraudCalculator,
    pub scorer: FraudScorer,
}

impl Default for FraudPipeline {
    fn default() -> Self {
        Self {
            policy: FraudPolicy::default(),
            calc: FraudCalculator::default(),
            scorer: FraudScorer::tuned(),
        }
    }
}

impl FraudPipeline {
    pub fn run_precheck(&self, rec: &mut FraudRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 943 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "fraud: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &FraudRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(&self, items: &mut [FraudRecord], weekday: bool) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = FraudScorer::combine(&scores);
        if combined > 893 {
            return Err(KernelError::Forbidden("fraud: batch risk too high".into()));
        }
        Ok(scores)
    }
}
