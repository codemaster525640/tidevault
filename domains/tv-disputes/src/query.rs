use crate::model::{DisputesFilter, DisputesRecord};
use crate::state::DisputesState;
use crate::store::DisputesStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct DisputesQuery;

impl DisputesQuery {
    pub fn by_merchant<'a>(
        store: &'a DisputesStore,
        merchant: MerchantId,
    ) -> Vec<&'a DisputesRecord> {
        store.query(&DisputesFilter {
            merchant_id: Some(merchant),
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn high_value<'a>(store: &'a DisputesStore, threshold: Decimal) -> Vec<&'a DisputesRecord> {
        store.query(&DisputesFilter {
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
        store: &'a DisputesStore,
        states: Vec<DisputesState>,
    ) -> Vec<&'a DisputesRecord> {
        store.query(&DisputesFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[DisputesRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 47;
        (count, sum, flagged)
    }
}
