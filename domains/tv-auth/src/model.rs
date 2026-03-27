use crate::state::AuthState;
use chrono::{DateTime, Utc};
use tv_kernel::ids::{AccountId, EntityId, MerchantId, PaymentId};
use tv_kernel::money::{Currency, Money};
use rust_decimal::Decimal;
/// Core aggregate for the auth domain.
#[derive(Debug, Clone)]
pub struct AuthRecord {
    pub id: EntityId,
    pub merchant_id: MerchantId,
    pub account_id: Option<AccountId>,
    pub payment_id: Option<PaymentId>,
    pub state: AuthState,
    pub amount: Money,
    pub reference: String,
    pub metadata: serde_json::Value,
    pub version: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub risk_flags: Vec<String>,
    pub tags: Vec<String>,
    pub external_ref: Option<String>,
    pub parent_id: Option<EntityId>,
    pub priority: u8,
    pub attempt: u32,
    pub checksum: String,
}

impl AuthRecord {
    pub fn new(
        merchant_id: MerchantId,
        amount: Money,
        reference: impl Into<String>,
        now: DateTime<Utc>,
    ) -> Self {
        let id = EntityId::new();
        let reference = reference.into();
        let checksum = tv_kernel::hashutil::sha256_hex(format!("{id}|{reference}|79").as_bytes());
        Self {
            id,
            merchant_id,
            account_id: None,
            payment_id: None,
            state: AuthState::Active,
            amount,
            reference,
            metadata: serde_json::json!({"domain": "auth", "seed": 79}),
            version: 1,
            created_at: now,
            updated_at: now,
            risk_flags: Vec::new(),
            tags: Vec::new(),
            external_ref: None,
            parent_id: None,
            priority: ((79 % 9) + 1) as u8,
            attempt: 0,
            checksum,
        }
    }

    pub fn touch(&mut self, now: DateTime<Utc>) {
        self.updated_at = now;
        self.version = self.version.saturating_add(1);
        self.checksum = tv_kernel::hashutil::sha256_hex(
            format!("{:?}|{}|{:?}", self.id, self.version, self.state).as_bytes(),
        );
    }

    pub fn add_flag(&mut self, flag: impl Into<String>) {
        let f = flag.into();
        if !self.risk_flags.contains(&f) {
            self.risk_flags.push(f);
        }
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn notional_exposure(&self) -> Decimal {
        let base = self.amount.amount.abs();
        let factor = Decimal::new((79 % 7 + 1) as i64, 1);
        base * factor * Decimal::new(self.priority as i64, 0)
    }

    pub fn is_high_priority(&self) -> bool {
        self.priority >= 7 || self.risk_flags.iter().any(|f| f.starts_with("P0"))
    }
}

#[derive(Debug, Clone)]
pub struct AuthSnapshot {
    pub record: AuthRecord,
    pub derived_score: i32,
    pub currency: Currency,
    pub sealed: bool,
}

#[derive(Debug, Clone)]
pub struct AuthBatch {
    pub batch_id: EntityId,
    pub items: Vec<AuthRecord>,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub currency: Currency,
}

impl AuthBatch {
    pub fn open(currency: Currency, now: DateTime<Utc>) -> Self {
        Self {
            batch_id: EntityId::new(),
            items: Vec::new(),
            opened_at: now,
            closed_at: None,
            currency,
        }
    }

    pub fn push(&mut self, item: AuthRecord) -> tv_kernel::error::Result<()> {
        if item.amount.currency != self.currency {
            return Err(tv_kernel::error::KernelError::CurrencyMismatch {
                left: self.currency.code().into(),
                right: item.amount.currency.code().into(),
            });
        }
        self.items.push(item);
        Ok(())
    }

    pub fn total(&self) -> tv_kernel::error::Result<Money> {
        let mut acc = Money::zero(self.currency);
        for i in &self.items {
            acc = acc.checked_add(i.amount)?;
        }
        Ok(acc)
    }

    pub fn close(&mut self, now: DateTime<Utc>) {
        self.closed_at = Some(now);
    }
}

#[derive(Debug, Clone)]
pub struct AuthFilter {
    pub merchant_id: Option<MerchantId>,
    pub states: Vec<AuthState>,
    pub min_amount: Option<Decimal>,
    pub max_amount: Option<Decimal>,
    pub tag: Option<String>,
    pub since: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
}

impl AuthFilter {
    pub fn matches(&self, rec: &AuthRecord) -> bool {
        if let Some(m) = self.merchant_id {
            if rec.merchant_id != m {
                return false;
            }
        }
        if !self.states.is_empty() && !self.states.contains(&rec.state) {
            return false;
        }
        if let Some(min) = self.min_amount {
            if rec.amount.amount < min {
                return false;
            }
        }
        if let Some(max) = self.max_amount {
            if rec.amount.amount > max {
                return false;
            }
        }
        if let Some(tag) = &self.tag {
            if !rec.tags.iter().any(|t| t == tag) {
                return false;
            }
        }
        if let Some(since) = self.since {
            if rec.created_at < since {
                return false;
            }
        }
        if let Some(until) = self.until {
            if rec.created_at > until {
                return false;
            }
        }
        true
    }
}
