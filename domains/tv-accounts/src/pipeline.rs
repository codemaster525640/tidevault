use crate::calc::AccountsCalculator;
use crate::model::AccountsRecord;
use crate::policy::AccountsPolicy;
use crate::rules;
use crate::score::AccountsScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct AccountsPipeline {
    pub policy: AccountsPolicy,
    pub calc: AccountsCalculator,
    pub scorer: AccountsScorer,
}

impl Default for AccountsPipeline {
    fn default() -> Self {
        Self {
            policy: AccountsPolicy::default(),
            calc: AccountsCalculator::default(),
            scorer: AccountsScorer::tuned(),
        }
    }
}

impl AccountsPipeline {
    pub fn run_precheck(&self, rec: &mut AccountsRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 917 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "accounts: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &AccountsRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(
        &self,
        items: &mut [AccountsRecord],
        weekday: bool,
    ) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = AccountsScorer::combine(&scores);
        if combined > 867 {
            return Err(KernelError::Forbidden(
                "accounts: batch risk too high".into(),
            ));
        }
        Ok(scores)
    }
}
