use crate::calc::WorkflowCalculator;
use crate::model::WorkflowRecord;
use crate::policy::WorkflowPolicy;
use crate::rules;
use crate::score::WorkflowScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct WorkflowPipeline {
    pub policy: WorkflowPolicy,
    pub calc: WorkflowCalculator,
    pub scorer: WorkflowScorer,
}

impl Default for WorkflowPipeline {
    fn default() -> Self {
        Self {
            policy: WorkflowPolicy::default(),
            calc: WorkflowCalculator::default(),
            scorer: WorkflowScorer::tuned(),
        }
    }
}

impl WorkflowPipeline {
    pub fn run_precheck(&self, rec: &mut WorkflowRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 997 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "workflow: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &WorkflowRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(
        &self,
        items: &mut [WorkflowRecord],
        weekday: bool,
    ) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = WorkflowScorer::combine(&scores);
        if combined > 947 {
            return Err(KernelError::Forbidden(
                "workflow: batch risk too high".into(),
            ));
        }
        Ok(scores)
    }
}
