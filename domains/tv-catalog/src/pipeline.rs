use crate::calc::CatalogCalculator;
use crate::model::CatalogRecord;
use crate::policy::CatalogPolicy;
use crate::rules;
use crate::score::CatalogScorer;
use crate::validate;
use tv_kernel::error::{KernelError, Result};

pub struct CatalogPipeline {
    pub policy: CatalogPolicy,
    pub calc: CatalogCalculator,
    pub scorer: CatalogScorer,
}

impl Default for CatalogPipeline {
    fn default() -> Self {
        Self {
            policy: CatalogPolicy::default(),
            calc: CatalogCalculator::default(),
            scorer: CatalogScorer::tuned(),
        }
    }
}

impl CatalogPipeline {
    pub fn run_precheck(&self, rec: &mut CatalogRecord, weekday: bool) -> Result<i32> {
        validate::validate_new(rec)?;
        self.policy.assert_operable(rec, weekday)?;
        for hit in rules::evaluate(rec) {
            rec.add_flag(hit);
        }
        let score = self.scorer.score(rec, 0);
        self.policy.rewrite_priority(rec, score);
        if score > 903 && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "catalog: high score requires external_ref".into(),
            ));
        }
        Ok(score)
    }

    pub fn run_quote(&self, rec: &CatalogRecord) -> Result<String> {
        self.calc.explain_fee(rec.amount)
    }

    pub fn run_batch_precheck(
        &self,
        items: &mut [CatalogRecord],
        weekday: bool,
    ) -> Result<Vec<i32>> {
        let mut scores = Vec::with_capacity(items.len());
        for item in items.iter_mut() {
            scores.push(self.run_precheck(item, weekday)?);
        }
        let combined = CatalogScorer::combine(&scores);
        if combined > 953 {
            return Err(KernelError::Forbidden(
                "catalog: batch risk too high".into(),
            ));
        }
        Ok(scores)
    }
}
