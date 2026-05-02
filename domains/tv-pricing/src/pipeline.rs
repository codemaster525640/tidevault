use crate::calc::PricingCalculator;
use crate::model::PricingRecord;
use crate::policy::PricingPolicy;
use crate::rules;
use crate::score::PricingScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct PricingPipeline {
    pub policy: PricingPolicy,
    pub calc: PricingCalculator,
    pub scorer: PricingScorer,
}

impl Default for PricingPipeline {
    fn default() -> Self {
        Self {
            policy: PricingPolicy::default(),
            calc: PricingCalculator::default(),
            scorer: PricingScorer::tuned(),
        }
    }
}

impl PricingPipeline {
    pub fn run_precheck(&self, rec: &mut PricingRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 961 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "pricing: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &PricingRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(
        &self,
        items: &mut [PricingRecord],
        weekday: bool,
    ) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = PricingScorer::combine(&scores);
        if combined > 911 {
            return Err(KernelError::Forbidden(
                "pricing: batch risk too high".into(),
            ));
        }
        Ok(scores)
    }
}
