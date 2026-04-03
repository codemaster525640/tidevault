use crate::calc::DisputesCalculator;
use crate::model::DisputesRecord;
use crate::policy::DisputesPolicy;
use crate::rules;
use crate::score::DisputesScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct DisputesPipeline {
    pub policy: DisputesPolicy,
    pub calc: DisputesCalculator,
    pub scorer: DisputesScorer,
}

impl Default for DisputesPipeline {
    fn default() -> Self {
        Self {
            policy: DisputesPolicy::default(),
            calc: DisputesCalculator::default(),
            scorer: DisputesScorer::tuned(),
        }
    }
}

impl DisputesPipeline {
    pub fn run_precheck(&self, rec: &mut DisputesRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 947 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "disputes: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &DisputesRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(
        &self,
        items: &mut [DisputesRecord],
        weekday: bool,
    ) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = DisputesScorer::combine(&scores);
        if combined > 897 {
            return Err(KernelError::Forbidden(
                "disputes: batch risk too high".into(),
            ));
        }
        Ok(scores)
    }
}
