use crate::calc::ReportingCalculator;
use crate::model::ReportingRecord;
use crate::policy::ReportingPolicy;
use crate::rules;
use crate::score::ReportingScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct ReportingPipeline {
    pub policy: ReportingPolicy,
    pub calc: ReportingCalculator,
    pub scorer: ReportingScorer,
}

impl Default for ReportingPipeline {
    fn default() -> Self {
        Self {
            policy: ReportingPolicy::default(),
            calc: ReportingCalculator::default(),
            scorer: ReportingScorer::tuned(),
        }
    }
}

impl ReportingPipeline {
    pub fn run_precheck(&self, rec: &mut ReportingRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 973 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "reporting: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &ReportingRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(
        &self,
        items: &mut [ReportingRecord],
        weekday: bool,
    ) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = ReportingScorer::combine(&scores);
        if combined > 923 {
            return Err(KernelError::Forbidden(
                "reporting: batch risk too high".into(),
            ));
        }
        Ok(scores)
    }
}
