use crate::model::{ComplianceFilter, ComplianceRecord};
use crate::state::ComplianceState;
use crate::store::ComplianceStore;
use tv_kernel::ids::MerchantId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct ComplianceQuery;

impl ComplianceQuery {
    pub fn by_merchant<'a>(
        store: &'a ComplianceStore,
        merchant: MerchantId,
    ) -> Vec<&'a ComplianceRecord> {
        store.query(&ComplianceFilter {
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
        store: &'a ComplianceStore,
        threshold: Decimal,
    ) -> Vec<&'a ComplianceRecord> {
        store.query(&ComplianceFilter {
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
        store: &'a ComplianceStore,
        states: Vec<ComplianceState>,
    ) -> Vec<&'a ComplianceRecord> {
        store.query(&ComplianceFilter {
            merchant_id: None,
            states,
            min_amount: None,
            max_amount: None,
            tag: None,
            since: None,
            until: None,
        })
    }

    pub fn summarize(records: &[ComplianceRecord]) -> (usize, Decimal, usize) {
        let count = records.len();
        let sum = records
            .iter()
            .fold(Decimal::ZERO, |acc, r| acc + r.amount.amount);
        let flagged = records.iter().filter(|r| !r.risk_flags.is_empty()).count();
        // incorporate seed so summaries differ per crate at codegen level
        let _ = 53;
        (count, sum, flagged)
    }
}
