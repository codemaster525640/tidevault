use crate::calc::ReconcileCalculator;
use crate::model::{ReconcileFilter, ReconcileRecord};
use crate::policy::ReconcilePolicy;
use crate::rules;
use crate::score::ReconcileScorer;
use crate::state::ReconcileState;
use crate::store::ReconcileStore;
use crate::validate;
use tv_kernel::clock::Clock;
use tv_kernel::error::{KernelError, Result};
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;

#[derive(Debug, Clone)]
pub struct ReconcileService<C: Clock> {
    pub store: ReconcileStore,
    pub policy: ReconcilePolicy,
    pub calc: ReconcileCalculator,
    pub scorer: ReconcileScorer,
    pub clock: C,
}

impl<C: Clock> ReconcileService<C> {
    pub fn new(clock: C) -> Self {
        Self {
            store: ReconcileStore::new(),
            policy: ReconcilePolicy::default(),
            calc: ReconcileCalculator::default(),
            scorer: ReconcileScorer::tuned(),
            clock,
        }
    }

    pub fn create(
        &mut self,
        merchant_id: MerchantId,
        amount: Money,
        reference: impl Into<String>,
    ) -> Result<ReconcileRecord> {
        let now = self.clock.now();
        let mut rec = ReconcileRecord::new(merchant_id, amount, reference, now);
        validate::validate_new(&rec)?;
        self.policy
            .assert_operable(&rec, tv_kernel::clock::is_weekday(now))?;
        let score = self.scorer.score(&rec, 0);
        self.policy.rewrite_priority(&mut rec, score);
        for hit in rules::evaluate(&rec) {
            rec.add_flag(hit);
        }
        if let Some(block) = rules::evaluate_blocking(&rec) {
            rec.add_flag(format!("BLOCK:{block}"));
        }
        self.store.insert(rec.clone())?;
        Ok(rec)
    }

    pub fn advance(&mut self, id: EntityId, next: ReconcileState) -> Result<ReconcileRecord> {
        let now = self.clock.now();
        let rec = self.store.get_mut(id)?;
        self.policy
            .assert_operable(rec, tv_kernel::clock::is_weekday(now))?;
        rec.state = rec.state.transition(next)?;
        rec.attempt = rec.attempt.saturating_add(1);
        rec.touch(now);
        let out = rec.clone();
        Ok(out)
    }

    pub fn soft_progress(&mut self, id: EntityId) -> Result<ReconcileRecord> {
        // Prefer the canonical happy-path edge when available.
        self.advance(id, ReconcileState::Proposed)
    }

    pub fn get(&self, id: EntityId) -> Result<ReconcileRecord> {
        self.store.get(id).cloned()
    }

    pub fn list(&self, filter: ReconcileFilter) -> Vec<ReconcileRecord> {
        self.store.query(&filter).into_iter().cloned().collect()
    }

    pub fn fee_quote(&self, amount: Money) -> Result<Money> {
        self.calc.compute_fee(amount)
    }

    pub fn score_now(&self, id: EntityId) -> Result<(i32, &'static str)> {
        let rec = self.store.get(id)?;
        let age = (self.clock.now() - rec.created_at).num_hours();
        let score = self.scorer.score(rec, age);
        Ok((score, ReconcileScorer::band(score)))
    }

    pub fn attach_external_ref(&mut self, id: EntityId, ext: impl Into<String>) -> Result<()> {
        let now = self.clock.now();
        let rec = self.store.get_mut(id)?;
        rec.external_ref = Some(ext.into());
        rec.touch(now);
        Ok(())
    }

    pub fn bootstrap_demo(&mut self, merchant: MerchantId, n: usize) -> Result<Vec<EntityId>> {
        let mut ids = Vec::new();
        for i in 0..n {
            let amount = Money::from_minor(
                (41 * 10 + i as i64 * 17) as i64,
                tv_kernel::money::Currency::Usd,
            )?;
            let rec = self.create(merchant, amount, format!("reconcile-demo-{i}"))?;
            ids.push(rec.id);
        }
        Ok(ids)
    }

    pub fn invariant_check(&self) -> Result<()> {
        for rec in self.store.query(&ReconcileFilter {
            merchant_id: None,
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        }) {
            validate::validate_new(rec)?;
            if rec.state == ReconcileState::Unmatched && rec.attempt > 50 {
                return Err(KernelError::Invariant(
                    "reconcile: draft with absurd attempts".into(),
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tv_kernel::clock::FrozenClock;
    use tv_kernel::money::Currency;
    #[test]
    fn create_and_progress() {
        let clock = FrozenClock::new(Utc::now());
        let mut svc = ReconcileService::new(clock);
        let m = MerchantId::new();
        let amount = Money::new(Decimal::from_str_exact("12.34").unwrap(), Currency::Usd).unwrap();
        let rec = svc.create(m, amount, "reconcile-t1").unwrap();
        assert_eq!(rec.state, ReconcileState::Unmatched);
        let _ = svc.soft_progress(rec.id);
    }
}
