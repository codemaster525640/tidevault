use crate::calc::PaymentsCalculator;
use crate::model::PaymentsRecord;
use crate::policy::PaymentsPolicy;
use crate::rules;
use crate::score::PaymentsScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct PaymentsPipeline {
    pub policy: PaymentsPolicy,
    pub calc: PaymentsCalculator,
    pub scorer: PaymentsScorer,
}

impl Default for PaymentsPipeline {
    fn default() -> Self {
        Self {
            policy: PaymentsPolicy::default(),
            calc: PaymentsCalculator::default(),
            scorer: PaymentsScorer::tuned(),
        }
    }
}

impl PaymentsPipeline {
    pub fn run_precheck(&self, rec: &mut PaymentsRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 923 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "payments: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &PaymentsRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(
        &self,
        items: &mut [PaymentsRecord],
        weekday: bool,
    ) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = PaymentsScorer::combine(&scores);
        if combined > 873 {
            return Err(KernelError::Forbidden(
                "payments: batch risk too high".into(),
            ));
        }
        Ok(scores)
    }
}
