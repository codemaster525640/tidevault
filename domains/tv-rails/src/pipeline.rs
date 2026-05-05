use crate::calc::RailsCalculator;
use crate::model::RailsRecord;
use crate::policy::RailsPolicy;
use crate::rules;
use crate::score::RailsScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct RailsPipeline {
    pub policy: RailsPolicy,
    pub calc: RailsCalculator,
    pub scorer: RailsScorer,
}

impl Default for RailsPipeline {
    fn default() -> Self {
        Self {
            policy: RailsPolicy::default(),
            calc: RailsCalculator::default(),
            scorer: RailsScorer::tuned(),
        }
    }
}

impl RailsPipeline {
    pub fn run_precheck(&self, rec: &mut RailsRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 929 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "rails: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &RailsRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(&self, items: &mut [RailsRecord], weekday: bool) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = RailsScorer::combine(&scores);
        if combined > 879 {
            return Err(KernelError::Forbidden("rails: batch risk too high".into()));
        }
        Ok(scores)
    }
}
