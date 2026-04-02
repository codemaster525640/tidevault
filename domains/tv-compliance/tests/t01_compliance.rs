use chrono::Utc;
use tv_compliance::calc::ComplianceCalculator;
use tv_compliance::model::ComplianceFilter;
use tv_compliance::pipeline::CompliancePipeline;
use tv_compliance::service::ComplianceService;
use tv_compliance::state::ComplianceState;
use tv_kernel::clock::FrozenClock;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::{Currency, Money};
use rust_decimal::Decimal;
#[test]
fn create_validate_and_score_1() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = ComplianceService::new(clock);
    let amount = Money::new(Decimal::from_str_exact("11.10").unwrap(), Currency::Usd).unwrap();
    let rec = svc
        .create(MerchantId::new(), amount, format!("compliance-u-1"))
        .unwrap();
    assert!(!rec.checksum.is_empty());
    let (score, band) = svc.score_now(rec.id).unwrap();
    assert!(score >= 0);
    assert!(["low", "medium", "high", "critical"].contains(&band));
}

#[test]
fn calculator_fee_non_negative_1() {
    let calc = ComplianceCalculator::default();
    let amount = Money::new(Decimal::from_str_exact("50.00").unwrap(), Currency::Usd).unwrap();
    let fee = calc.compute_fee(amount).unwrap();
    assert!(!fee.is_negative());
}

#[test]
fn pipeline_precheck_1() {
    let pipe = CompliancePipeline::default();
    let mut rec = tv_compliance::model::ComplianceRecord::new(
        MerchantId::new(),
        Money::new(Decimal::from_str_exact("20.00").unwrap(), Currency::Usd).unwrap(),
        format!("compliance-p-1"),
        Utc::now(),
    );
    let score = pipe.run_precheck(&mut rec, true).unwrap();
    assert!(score >= 0);
}

#[test]
fn list_filter_1() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = ComplianceService::new(clock);
    let m = MerchantId::new();
    let _ = svc
        .create(
            m,
            Money::new(Decimal::from_str_exact("1.00").unwrap(), Currency::Usd).unwrap(),
            format!("compliance-l-1"),
        )
        .unwrap();
    let items = svc.list(ComplianceFilter {
        merchant_id: Some(m),
        states: vec![ComplianceState::Pass],
        min_amount: None,
        max_amount: None,
        tag: None,
        since: None,
        until: None,
    });
    assert!(!items.is_empty());
}
