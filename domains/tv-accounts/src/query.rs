use crate::model::{AccountsFilter, AccountsRecord};
use crate::state::AccountsState;
use crate::store::AccountsStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct AccountsQuery;

impl AccountsQuery {
    pub fn by_merchant<'a>(
        store: &'a AccountsStore,
        merchant: MerchantId,
    ) -> Vec<&'a AccountsRecord> {
        store.query(&AccountsFilter {
            merchant_id: Some(merchant),
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn high_value<'a>(store: &'a AccountsStore, threshold: Decimal) -> Vec<&'a AccountsRecord> {
        store.query(&AccountsFilter {
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
        store: &'a AccountsStore,
        states: Vec<AccountsState>,
    ) -> Vec<&'a AccountsRecord> {
        store.query(&AccountsFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[AccountsRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 17;
        (count, sum, flagged)
    }
}
