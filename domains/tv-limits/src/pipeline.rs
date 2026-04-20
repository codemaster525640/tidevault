use crate::calc::LimitsCalculator;
use crate::model::LimitsRecord;
use crate::policy::LimitsPolicy;
use crate::rules;
use crate::score::LimitsScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct LimitsPipeline {
    pub policy: LimitsPolicy,
    pub calc: LimitsCalculator,
    pub scorer: LimitsScorer,
}

impl Default for LimitsPipeline {
    fn default() -> Self {
        Self {
            policy: LimitsPolicy::default(),
            calc: LimitsCalculator::default(),
            scorer: LimitsScorer::tuned(),
        }
    }
}

impl LimitsPipeline {
    pub fn run_precheck(&self, rec: &mut LimitsRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 967 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "limits: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &LimitsRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(
        &self,
        items: &mut [LimitsRecord],
        weekday: bool,
    ) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = LimitsScorer::combine(&scores);
        if combined > 917 {
            return Err(KernelError::Forbidden("limits: batch risk too high".into()));
        }
        Ok(scores)
    }
}
