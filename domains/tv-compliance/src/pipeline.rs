use crate::calc::ComplianceCalculator;
use crate::model::ComplianceRecord;
use crate::policy::CompliancePolicy;
use crate::rules;
use crate::score::ComplianceScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct CompliancePipeline {
    pub policy: CompliancePolicy,
    pub calc: ComplianceCalculator,
    pub scorer: ComplianceScorer,
}

impl Default for CompliancePipeline {
    fn default() -> Self {
        Self {
            policy: CompliancePolicy::default(),
            calc: ComplianceCalculator::default(),
            scorer: ComplianceScorer::tuned(),
        }
    }
}

impl CompliancePipeline {
    pub fn run_precheck(&self, rec: &mut ComplianceRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 953 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "compliance: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &ComplianceRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(
        &self,
        items: &mut [ComplianceRecord],
        weekday: bool,
    ) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = ComplianceScorer::combine(&scores);
        if combined > 903 {
            return Err(KernelError::Forbidden(
                "compliance: batch risk too high".into(),
            ));
        }
        Ok(scores)
    }
}
