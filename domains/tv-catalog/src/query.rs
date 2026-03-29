use crate::model::{CatalogFilter, CatalogRecord};
use crate::state::CatalogState;
use crate::store::CatalogStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct CatalogQuery;

impl CatalogQuery {
    pub fn by_merchant<'a>(
        store: &'a CatalogStore,
        merchant: MerchantId,
    ) -> Vec<&'a CatalogRecord> {
        store.query(&CatalogFilter {
            merchant_id: Some(merchant),
            states: vec![],
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn high_value<'a>(store: &'a CatalogStore, threshold: Decimal) -> Vec<&'a CatalogRecord> {
        store.query(&CatalogFilter {
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
        store: &'a CatalogStore,
        states: Vec<CatalogState>,
    ) -> Vec<&'a CatalogRecord> {
        store.query(&CatalogFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[CatalogRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 103;
        (count, sum, flagged)
    }
}
