use crate::model::{RoutingFilter, RoutingRecord};
use crate::state::RoutingState;
use crate::store::RoutingStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct RoutingQuery;

impl RoutingQuery {
    pub fn by_merchant<'a>(
        store: &'a RoutingStore,
        merchant: MerchantId,
    ) -> Vec<&'a RoutingRecord> {
        store.query(&RoutingFilter {
            merchant_id: Some(merchant),
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn high_value<'a>(store: &'a RoutingStore, threshold: Decimal) -> Vec<&'a RoutingRecord> {
        store.query(&RoutingFilter {
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
        store: &'a RoutingStore,
        states: Vec<RoutingState>,
    ) -> Vec<&'a RoutingRecord> {
        store.query(&RoutingFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[RoutingRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 71;
        (count, sum, flagged)
    }
}
