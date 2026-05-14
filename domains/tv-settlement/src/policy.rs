use crate::model::SettlementRecord;
use crate::state::SettlementState;
use tv_kernel::error::{KernelError, Result};
use tv_kernel::money::Money;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct SettlementPolicy {
    pub max_attempt: u32,
    pub require_external_ref_after_attempt: u32,
    pub block_currencies: Vec<&'static str>,
    pub allow_weekend_ops: bool,
    pub max_notional: Decimal,
    pub auto_escalate_score: i32,
}

impl Default for SettlementPolicy {
    fn default() -> Self {
        Self {
            max_attempt: 5,
            require_external_ref_after_attempt: 3,
            block_currencies: if 37 % 2 == 0 {
                vec!["TRY"]
            } else {
                vec!["AED"]
            },
            allow_weekend_ops: false,
            max_notional: Decimal::new(253700, 0),
            auto_escalate_score: 637,
        }
    }
}

impl SettlementPolicy {
    pub fn assert_operable(&self, rec: &SettlementRecord, weekday: bool) -> Result<()> {
        if !weekday && !self.allow_weekend_ops {
            return Err(KernelError::Forbidden(
                "settlement: weekend ops disabled".into(),
            ));
        }
        if rec.attempt >= self.max_attempt {
            return Err(KernelError::Conflict(format!(
                "settlement: attempts exhausted ({})",
                rec.attempt
            )));
        }
        if rec.attempt >= self.require_external_ref_after_attempt && rec.external_ref.is_none() {
            return Err(KernelError::Validation(
                "settlement: external_ref required after retries".into(),
            ));
        }
        let code = rec.amount.currency.code();
        if self.block_currencies.iter().any(|c| *c == code) {
            return Err(KernelError::Forbidden(format!(
                "settlement: currency {code} blocked by policy"
            )));
        }
        if rec.amount.amount > self.max_notional {
            return Err(KernelError::Forbidden(
                "settlement: exceeds max notional".into(),
            ));
        }
        Ok(())
    }

    pub fn allowed_next(&self, from: SettlementState, to: SettlementState) -> Result<()> {
        from.transition(to).map(|_| ())
    }

    pub fn should_escalate(&self, score: i32, rec: &SettlementRecord) -> bool {
        score >= self.auto_escalate_score || rec.is_high_priority()
    }

    pub fn cooldown_seconds(&self, rec: &SettlementRecord) -> u64 {
        let base = 67u64;
        base.saturating_mul(1 + rec.attempt as u64).min(3600)
    }

    pub fn rewrite_priority(&self, rec: &mut SettlementRecord, score: i32) {
        if score > self.auto_escalate_score {
            rec.priority = rec.priority.max(8);
            rec.add_flag("POLICY_ESCALATE");
        } else if score < self.auto_escalate_score / 3 {
            rec.priority = rec.priority.min(3);
        }
    }

    pub fn fee_waiver_eligible(&self, amount: Money, loyalty_tier: u8) -> bool {
        loyalty_tier >= 4 && amount.amount >= Decimal::new(370, 0)
    }
}
