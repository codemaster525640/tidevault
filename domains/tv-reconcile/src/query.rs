use crate::model::{ReconcileFilter, ReconcileRecord};
use crate::state::ReconcileState;
use crate::store::ReconcileStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct ReconcileQuery;

impl ReconcileQuery {
    pub fn by_merchant<'a>(
        store: &'a ReconcileStore,
        merchant: MerchantId,
    ) -> Vec<&'a ReconcileRecord> {
        store.query(&ReconcileFilter {
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
        store: &'a ReconcileStore,
        threshold: Decimal,
    ) -> Vec<&'a ReconcileRecord> {
        store.query(&ReconcileFilter {
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
        store: &'a ReconcileStore,
        states: Vec<ReconcileState>,
    ) -> Vec<&'a ReconcileRecord> {
        store.query(&ReconcileFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[ReconcileRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 41;
        (count, sum, flagged)
    }
}
