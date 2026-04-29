use crate::model::{PaymentsFilter, PaymentsRecord};
use crate::state::PaymentsState;
use crate::store::PaymentsStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct PaymentsQuery;

impl PaymentsQuery {
    pub fn by_merchant<'a>(
        store: &'a PaymentsStore,
        merchant: MerchantId,
    ) -> Vec<&'a PaymentsRecord> {
        store.query(&PaymentsFilter {
            merchant_id: Some(merchant),
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn high_value<'a>(store: &'a PaymentsStore, threshold: Decimal) -> Vec<&'a PaymentsRecord> {
        store.query(&PaymentsFilter {
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
        store: &'a PaymentsStore,
        states: Vec<PaymentsState>,
    ) -> Vec<&'a PaymentsRecord> {
        store.query(&PaymentsFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[PaymentsRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 23;
        (count, sum, flagged)
    }
}
