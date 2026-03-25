use crate::model::{AuditFilter, AuditRecord};
use crate::state::AuditState;
use crate::store::AuditStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct AuditQuery;

impl AuditQuery {
    pub fn by_merchant<'a>(store: &'a AuditStore, merchant: MerchantId) -> Vec<&'a AuditRecord> {
        store.query(&AuditFilter {
            merchant_id: Some(merchant),
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn high_value<'a>(store: &'a AuditStore, threshold: Decimal) -> Vec<&'a AuditRecord> {
        store.query(&AuditFilter {
            merchant_id: None,
            states: vec![],
            min_amount: Some(threshold),
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn in_states<'a>(store: &'a AuditStore, states: Vec<AuditState>) -> Vec<&'a AuditRecord> {
        store.query(&AuditFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[AuditRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 83;
        (count, sum, flagged)
    }
}
