use crate::model::{ReportingFilter, ReportingRecord};
use crate::state::ReportingState;
use crate::store::ReportingStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct ReportingQuery;

impl ReportingQuery {
    pub fn by_merchant<'a>(
        store: &'a ReportingStore,
        merchant: MerchantId,
    ) -> Vec<&'a ReportingRecord> {
        store.query(&ReportingFilter {
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
        store: &'a ReportingStore,
        threshold: Decimal,
    ) -> Vec<&'a ReportingRecord> {
        store.query(&ReportingFilter {
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
        store: &'a ReportingStore,
        states: Vec<ReportingState>,
    ) -> Vec<&'a ReportingRecord> {
        store.query(&ReportingFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[ReportingRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 73;
        (count, sum, flagged)
    }
}
