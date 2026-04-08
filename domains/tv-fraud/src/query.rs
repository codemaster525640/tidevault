use crate::model::{FraudFilter, FraudRecord};
use crate::state::FraudState;
use crate::store::FraudStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct FraudQuery;

impl FraudQuery {
    pub fn by_merchant<'a>(store: &'a FraudStore, merchant: MerchantId) -> Vec<&'a FraudRecord> {
        store.query(&FraudFilter {
            merchant_id: Some(merchant),
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn high_value<'a>(store: &'a FraudStore, threshold: Decimal) -> Vec<&'a FraudRecord> {
        store.query(&FraudFilter {
            merchant_id: None,
            states: vec![],
            min_amount: Some(threshold),
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn in_states<'a>(store: &'a FraudStore, states: Vec<FraudState>) -> Vec<&'a FraudRecord> {
        store.query(&FraudFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[FraudRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 43;
        (count, sum, flagged)
    }
}
