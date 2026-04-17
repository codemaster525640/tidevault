use crate::calc::FxCalculator;
use crate::model::FxRecord;
use crate::policy::FxPolicy;
use crate::rules;
use crate::score::FxScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct FxPipeline {
    pub policy: FxPolicy,
    pub calc: FxCalculator,
    pub scorer: FxScorer,
}

impl Default for FxPipeline {
    fn default() -> Self {
        Self {
            policy: FxPolicy::default(),
            calc: FxCalculator::default(),
            scorer: FxScorer::tuned(),
        }
    }
}

impl FxPipeline {
    pub fn run_precheck(&self, rec: &mut FxRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 931 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "fx: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &FxRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(&self, items: &mut [FxRecord], weekday: bool) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = FxScorer::combine(&scores);
        if combined > 881 {
            return Err(KernelError::Forbidden("fx: batch risk too high".into()));
        }
        Ok(scores)
    }
}
