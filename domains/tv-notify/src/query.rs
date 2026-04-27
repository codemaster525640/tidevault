use crate::model::{NotifyFilter, NotifyRecord};
use crate::state::NotifyState;
use crate::store::NotifyStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct NotifyQuery;

impl NotifyQuery {
    pub fn by_merchant<'a>(store: &'a NotifyStore, merchant: MerchantId) -> Vec<&'a NotifyRecord> {
        store.query(&NotifyFilter {
            merchant_id: Some(merchant),
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn high_value<'a>(store: &'a NotifyStore, threshold: Decimal) -> Vec<&'a NotifyRecord> {
        store.query(&NotifyFilter {
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
        store: &'a NotifyStore,
        states: Vec<NotifyState>,
    ) -> Vec<&'a NotifyRecord> {
        store.query(&NotifyFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[NotifyRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 89;
        (count, sum, flagged)
    }
}
