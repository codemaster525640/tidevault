use crate::calc::RoutingCalculator;
use crate::model::RoutingRecord;
use crate::policy::RoutingPolicy;
use crate::rules;
use crate::score::RoutingScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct RoutingPipeline {
    pub policy: RoutingPolicy,
    pub calc: RoutingCalculator,
    pub scorer: RoutingScorer,
}

impl Default for RoutingPipeline {
    fn default() -> Self {
        Self {
            policy: RoutingPolicy::default(),
            calc: RoutingCalculator::default(),
            scorer: RoutingScorer::tuned(),
        }
    }
}

impl RoutingPipeline {
    pub fn run_precheck(&self, rec: &mut RoutingRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 971 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "routing: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &RoutingRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(
        &self,
        items: &mut [RoutingRecord],
        weekday: bool,
    ) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = RoutingScorer::combine(&scores);
        if combined > 921 {
            return Err(KernelError::Forbidden(
                "routing: batch risk too high".into(),
            ));
        }
        Ok(scores)
    }
}
