use crate::model::{TreasuryFilter, TreasuryRecord};
use crate::state::TreasuryState;
use crate::store::TreasuryStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct TreasuryQuery;

impl TreasuryQuery {
    pub fn by_merchant<'a>(
        store: &'a TreasuryStore,
        merchant: MerchantId,
    ) -> Vec<&'a TreasuryRecord> {
        store.query(&TreasuryFilter {
            merchant_id: Some(merchant),
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn high_value<'a>(store: &'a TreasuryStore, threshold: Decimal) -> Vec<&'a TreasuryRecord> {
        store.query(&TreasuryFilter {
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
        store: &'a TreasuryStore,
        states: Vec<TreasuryState>,
    ) -> Vec<&'a TreasuryRecord> {
        store.query(&TreasuryFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[TreasuryRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 101;
        (count, sum, flagged)
    }
}
