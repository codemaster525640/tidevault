use crate::calc::NotifyCalculator;
use crate::model::NotifyRecord;
use crate::policy::NotifyPolicy;
use crate::rules;
use crate::score::NotifyScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct NotifyPipeline {
    pub policy: NotifyPolicy,
    pub calc: NotifyCalculator,
    pub scorer: NotifyScorer,
}

impl Default for NotifyPipeline {
    fn default() -> Self {
        Self {
            policy: NotifyPolicy::default(),
            calc: NotifyCalculator::default(),
            scorer: NotifyScorer::tuned(),
        }
    }
}

impl NotifyPipeline {
    pub fn run_precheck(&self, rec: &mut NotifyRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 989 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "notify: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &NotifyRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(
        &self,
        items: &mut [NotifyRecord],
        weekday: bool,
    ) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = NotifyScorer::combine(&scores);
        if combined > 939 {
            return Err(KernelError::Forbidden("notify: batch risk too high".into()));
        }
        Ok(scores)
    }
}
