use crate::model::{LedgerFilter, LedgerRecord};
use crate::state::LedgerState;
use crate::store::LedgerStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct LedgerQuery;

impl LedgerQuery {
    pub fn by_merchant<'a>(store: &'a LedgerStore, merchant: MerchantId) -> Vec<&'a LedgerRecord> {
        store.query(&LedgerFilter {
            merchant_id: Some(merchant),
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn high_value<'a>(store: &'a LedgerStore, threshold: Decimal) -> Vec<&'a LedgerRecord> {
        store.query(&LedgerFilter {
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
        store: &'a LedgerStore,
        states: Vec<LedgerState>,
    ) -> Vec<&'a LedgerRecord> {
        store.query(&LedgerFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[LedgerRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 11;
        (count, sum, flagged)
    }
}
