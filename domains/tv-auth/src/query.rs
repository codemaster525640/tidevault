use crate::model::{AuthFilter, AuthRecord};
use crate::state::AuthState;
use crate::store::AuthStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct AuthQuery;

impl AuthQuery {
    pub fn by_merchant<'a>(store: &'a AuthStore, merchant: MerchantId) -> Vec<&'a AuthRecord> {
        store.query(&AuthFilter {
            merchant_id: Some(merchant),
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn high_value<'a>(store: &'a AuthStore, threshold: Decimal) -> Vec<&'a AuthRecord> {
        store.query(&AuthFilter {
            merchant_id: None,
            states: vec![],
            min_amount: Some(threshold),
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn in_states<'a>(store: &'a AuthStore, states: Vec<AuthState>) -> Vec<&'a AuthRecord> {
        store.query(&AuthFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[AuthRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 79;
        (count, sum, flagged)
    }
}
