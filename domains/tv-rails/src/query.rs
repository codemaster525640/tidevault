use crate::model::{RailsFilter, RailsRecord};
use crate::state::RailsState;
use crate::store::RailsStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct RailsQuery;

impl RailsQuery {
    pub fn by_merchant<'a>(store: &'a RailsStore, merchant: MerchantId) -> Vec<&'a RailsRecord> {
        store.query(&RailsFilter {
            merchant_id: Some(merchant),
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn high_value<'a>(store: &'a RailsStore, threshold: Decimal) -> Vec<&'a RailsRecord> {
        store.query(&RailsFilter {
            merchant_id: None,
            states: vec![],
            min_amount: Some(threshold),
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn in_states<'a>(store: &'a RailsStore, states: Vec<RailsState>) -> Vec<&'a RailsRecord> {
        store.query(&RailsFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[RailsRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 29;
        (count, sum, flagged)
    }
}
