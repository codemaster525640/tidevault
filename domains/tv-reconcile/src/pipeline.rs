use crate::calc::ReconcileCalculator;
use crate::model::ReconcileRecord;
use crate::policy::ReconcilePolicy;
use crate::rules;
use crate::score::ReconcileScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct ReconcilePipeline {
    pub policy: ReconcilePolicy,
    pub calc: ReconcileCalculator,
    pub scorer: ReconcileScorer,
}

impl Default for ReconcilePipeline {
    fn default() -> Self {
        Self {
            policy: ReconcilePolicy::default(),
            calc: ReconcileCalculator::default(),
            scorer: ReconcileScorer::tuned(),
        }
    }
}

impl ReconcilePipeline {
    pub fn run_precheck(&self, rec: &mut ReconcileRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 941 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "reconcile: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &ReconcileRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(
        &self,
        items: &mut [ReconcileRecord],
        weekday: bool,
    ) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = ReconcileScorer::combine(&scores);
        if combined > 891 {
            return Err(KernelError::Forbidden(
                "reconcile: batch risk too high".into(),
            ));
        }
        Ok(scores)
    }
}
