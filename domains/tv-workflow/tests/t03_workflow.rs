use chrono::Utc;
use tv_kernel::clock::FrozenClock;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::{Currency, Money};
use tv_workflow::calc::WorkflowCalculator;
use tv_workflow::model::WorkflowFilter;
use tv_workflow::pipeline::WorkflowPipeline;
use tv_workflow::service::WorkflowService;
use tv_workflow::state::WorkflowState;
use rust_decimal::Decimal;
#[test]
fn create_validate_and_score_3() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = WorkflowService::new(clock);
    let amount = Money::new(Decimal::from_str_exact("13.30").unwrap(), Currency::Usd).unwrap();
    let rec = svc
        .create(MerchantId::new(), amount, format!("workflow-u-3"))
        .unwrap();
    assert!(!rec.checksum.is_empty());
    let (score, band) = svc.score_now(rec.id).unwrap();
    assert!(score >= 0);
    assert!(["low", "medium", "high", "critical"].contains(&band));
}

#[test]
fn calculator_fee_non_negative_3() {
    let calc = WorkflowCalculator::default();
    let amount = Money::new(Decimal::from_str_exact("150.00").unwrap(), Currency::Usd).unwrap();
    let fee = calc.compute_fee(amount).unwrap();
    assert!(!fee.is_negative());
}

#[test]
fn pipeline_precheck_3() {
    let pipe = WorkflowPipeline::default();
    let mut rec = tv_workflow::model::WorkflowRecord::new(
        MerchantId::new(),
        Money::new(Decimal::from_str_exact("20.00").unwrap(), Currency::Usd).unwrap(),
        format!("workflow-p-3"),
        Utc::now(),
    );
    let score = pipe.run_precheck(&mut rec, true).unwrap();
    assert!(score >= 0);
}

#[test]
fn list_filter_3() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = WorkflowService::new(clock);
    let m = MerchantId::new();
    let _ = svc
        .create(
            m,
            Money::new(Decimal::from_str_exact("1.00").unwrap(), Currency::Usd).unwrap(),
            format!("workflow-l-3"),
        )
        .unwrap();
    let items = svc.list(WorkflowFilter {
        merchant_id: Some(m),
        states: vec![WorkflowState::Started],
        min_amount: None,
        max_amount: None,
        tag: None,
        since: None,
        until: None,
    });
    assert!(!items.is_empty());
}
