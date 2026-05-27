use crate::calc::WebhooksCalculator;
use crate::model::WebhooksRecord;
use crate::policy::WebhooksPolicy;
use crate::rules;
use crate::score::WebhooksScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct WebhooksPipeline {
    pub policy: WebhooksPolicy,
    pub calc: WebhooksCalculator,
    pub scorer: WebhooksScorer,
}

impl Default for WebhooksPipeline {
    fn default() -> Self {
        Self {
            policy: WebhooksPolicy::default(),
            calc: WebhooksCalculator::default(),
            scorer: WebhooksScorer::tuned(),
        }
    }
}

impl WebhooksPipeline {
    pub fn run_precheck(&self, rec: &mut WebhooksRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 959 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "webhooks: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &WebhooksRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(
        &self,
        items: &mut [WebhooksRecord],
        weekday: bool,
    ) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = WebhooksScorer::combine(&scores);
        if combined > 909 {
            return Err(KernelError::Forbidden(
                "webhooks: batch risk too high".into(),
            ));
        }
        Ok(scores)
    }
}
