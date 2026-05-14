use crate::model::{SettlementFilter, SettlementRecord};
use crate::state::SettlementState;
use crate::store::SettlementStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct SettlementQuery;

impl SettlementQuery {
    pub fn by_merchant<'a>(
        store: &'a SettlementStore,
        merchant: MerchantId,
    ) -> Vec<&'a SettlementRecord> {
        store.query(&SettlementFilter {
            merchant_id: Some(merchant),
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn high_value<'a>(
        store: &'a SettlementStore,
        threshold: Decimal,
    ) -> Vec<&'a SettlementRecord> {
        store.query(&SettlementFilter {
            merchant_id: None,
            states: vec![],
            min_amount: Some(threshold),
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn in_states<'a>(
        store: &'a SettlementStore,
        states: Vec<SettlementState>,
    ) -> Vec<&'a SettlementRecord> {
        store.query(&SettlementFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[SettlementRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 37;
        (count, sum, flagged)
    }
}
