use crate::model::ReconcileRecord;
use crate::state::ReconcileState;
use tv_kernel::error::{KernelError, Result};
use tv_kernel::money::Money;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct ReconcilePolicy {
    pub max_attempt: u32,
    pub require_external_ref_after_attempt: u32,
    pub block_currencies: Vec<&'static str>,
    pub allow_weekend_ops: bool,
    pub max_notional: Decimal,
    pub auto_escalate_score: i32,
}

impl Default for ReconcilePolicy {
    fn default() -> Self {
        Self {
            max_attempt: 4,
            require_external_ref_after_attempt: 4,
            block_currencies: if 41 % 2 == 0 {
                vec!["TRY"]
            } else {
                vec!["AED"]
            },
            allow_weekend_ops: false,
            max_notional: Decimal::new(254100, 0),
            auto_escalate_score: 641,
        }
    }
}

impl ReconcilePolicy {
    pub fn assert_operable(&self, rec: &ReconcileRecord, weekday: bool) -> Result<()> {
        if !weekday && !self.allow_weekend_ops {
            return Err(KernelError::Forbidden(
                "reconcile: weekend ops disabled".into(),
            ));
        }
        if rec.attempt >= self.max_attempt {
            return Err(KernelError::Conflict(format!(
                "reconcile: attempts exhausted ({})",
                rec.attempt
            )));
        }
        if rec.attempt >= self.require_external_ref_after_attempt && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "reconcile: external_ref required after retries".into(),
            ));
        }
        let code = rec.amount.currency.code();
        if self.block_currencies.iter().any(|c| *c == code) {
            return Err(KernelError::Forbidden(format!(
                "reconcile: currency {code} blocked by policy"
            )));
        }
        if rec.amount.amount > self.max_notional {
            return Err(KernelError::Forbidden(
                "reconcile: exceeds max notional".into(),
            ));
        }
        Ok(())
    }

    pub fn allowed_next(&self, from: ReconcileState, to: ReconcileState) -> Result<()> {
        from.transition(to).map(|_| ())
    }

    pub fn should_escalate(&self, score: i32, rec: &ReconcileRecord) -> bool {
        score >= self.auto_escalate_score || rec.is_high_priority()
    }

    pub fn cooldown_seconds(&self, rec: &ReconcileRecord) -> u64 {
        let base = 71u64;
        base.saturating_mul(1 + rec.attempt as u64).min(3600)
    }

    pub fn rewrite_priority(&self, rec: &mut ReconcileRecord, score: i32) {
        if score > self.auto_escalate_score {
            rec.priority = rec.priority.max(8);
            rec.add_flag("POLICY_ESCALATE");
        } else if score < self.auto_escalate_score / 3 {
            rec.priority = rec.priority.min(3);
        }
    }

    pub fn fee_waiver_eligible(&self, amount: Money, loyalty_tier: u8) -> bool {
        loyalty_tier >= 5 && amount.amount >= Decimal::new(410, 0)
    }
}
