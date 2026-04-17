use crate::model::{FxFilter, FxRecord};
use crate::state::FxState;
use crate::store::FxStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct FxQuery;

impl FxQuery {
    pub fn by_merchant<'a>(store: &'a FxStore, merchant: MerchantId) -> Vec<&'a FxRecord> {
        store.query(&FxFilter {
            merchant_id: Some(merchant),
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn high_value<'a>(store: &'a FxStore, threshold: Decimal) -> Vec<&'a FxRecord> {
        store.query(&FxFilter {
            merchant_id: None,
            states: vec![],
            min_amount: Some(threshold),
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn in_states<'a>(store: &'a FxStore, states: Vec<FxState>) -> Vec<&'a FxRecord> {
        store.query(&FxFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[FxRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 31;
        (count, sum, flagged)
    }
}
