use crate::model::{LimitsFilter, LimitsRecord};
use crate::state::LimitsState;
use crate::store::LimitsStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct LimitsQuery;

impl LimitsQuery {
    pub fn by_merchant<'a>(store: &'a LimitsStore, merchant: MerchantId) -> Vec<&'a LimitsRecord> {
        store.query(&LimitsFilter {
            merchant_id: Some(merchant),
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn high_value<'a>(store: &'a LimitsStore, threshold: Decimal) -> Vec<&'a LimitsRecord> {
        store.query(&LimitsFilter {
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
        store: &'a LimitsStore,
        states: Vec<LimitsState>,
    ) -> Vec<&'a LimitsRecord> {
        store.query(&LimitsFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[LimitsRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 67;
        (count, sum, flagged)
    }
}
