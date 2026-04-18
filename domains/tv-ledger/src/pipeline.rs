use crate::calc::LedgerCalculator;
use crate::model::LedgerRecord;
use crate::policy::LedgerPolicy;
use crate::rules;
use crate::score::LedgerScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct LedgerPipeline {
    pub policy: LedgerPolicy,
    pub calc: LedgerCalculator,
    pub scorer: LedgerScorer,
}

impl Default for LedgerPipeline {
    fn default() -> Self {
        Self {
            policy: LedgerPolicy::default(),
            calc: LedgerCalculator::default(),
            scorer: LedgerScorer::tuned(),
        }
    }
}

impl LedgerPipeline {
    pub fn run_precheck(&self, rec: &mut LedgerRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 911 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "ledger: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &LedgerRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(
        &self,
        items: &mut [LedgerRecord],
        weekday: bool,
    ) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = LedgerScorer::combine(&scores);
        if combined > 861 {
            return Err(KernelError::Forbidden("ledger: batch risk too high".into()));
        }
        Ok(scores)
    }
}
