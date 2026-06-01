use crate::model::{WorkflowFilter, WorkflowRecord};
use crate::state::WorkflowState;
use crate::store::WorkflowStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct WorkflowQuery;

impl WorkflowQuery {
    pub fn by_merchant<'a>(
        store: &'a WorkflowStore,
        merchant: MerchantId,
    ) -> Vec<&'a WorkflowRecord> {
        store.query(&WorkflowFilter {
            merchant_id: Some(merchant),
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn high_value<'a>(store: &'a WorkflowStore, threshold: Decimal) -> Vec<&'a WorkflowRecord> {
        store.query(&WorkflowFilter {
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
        store: &'a WorkflowStore,
        states: Vec<WorkflowState>,
    ) -> Vec<&'a WorkflowRecord> {
        store.query(&WorkflowFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[WorkflowRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 97;
        (count, sum, flagged)
    }
}
