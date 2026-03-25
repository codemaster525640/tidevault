use crate::calc::AuditCalculator;
use crate::model::AuditRecord;
use crate::policy::AuditPolicy;
use crate::rules;
use crate::score::AuditScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct AuditPipeline {
    pub policy: AuditPolicy,
    pub calc: AuditCalculator,
    pub scorer: AuditScorer,
}

impl Default for AuditPipeline {
    fn default() -> Self {
        Self {
            policy: AuditPolicy::default(),
            calc: AuditCalculator::default(),
            scorer: AuditScorer::tuned(),
        }
    }
}

impl AuditPipeline {
    pub fn run_precheck(&self, rec: &mut AuditRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 983 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "audit: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &AuditRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(&self, items: &mut [AuditRecord], weekday: bool) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = AuditScorer::combine(&scores);
        if combined > 933 {
            return Err(KernelError::Forbidden("audit: batch risk too high".into()));
        }
        Ok(scores)
    }
}
