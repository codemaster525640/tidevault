use crate::calc::AuthCalculator;
use crate::model::AuthRecord;
use crate::policy::AuthPolicy;
use crate::rules;
use crate::score::AuthScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct AuthPipeline {
    pub policy: AuthPolicy,
    pub calc: AuthCalculator,
    pub scorer: AuthScorer,
}

impl Default for AuthPipeline {
    fn default() -> Self {
        Self {
            policy: AuthPolicy::default(),
            calc: AuthCalculator::default(),
            scorer: AuthScorer::tuned(),
        }
    }
}

impl AuthPipeline {
    pub fn run_precheck(&self, rec: &mut AuthRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 979 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "auth: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &AuthRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(&self, items: &mut [AuthRecord], weekday: bool) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = AuthScorer::combine(&scores);
        if combined > 929 {
            return Err(KernelError::Forbidden("auth: batch risk too high".into()));
        }
        Ok(scores)
    }
}
