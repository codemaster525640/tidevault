use chrono::Utc;
use tv_fraud::calc::FraudCalculator;
use tv_fraud::model::FraudFilter;
use tv_fraud::pipeline::FraudPipeline;
use tv_fraud::service::FraudService;
use tv_fraud::state::FraudState;
use tv_kernel::clock::FrozenClock;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::{Currency, Money};
use rust_decimal::Decimal;
#[test]
fn create_validate_and_score_2() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = FraudService::new(clock);
    let amount = Money::new(Decimal::from_str_exact("12.20").unwrap(), Currency::Usd).unwrap();
    let rec = svc
        .create(MerchantId::new(), amount, format!("fraud-u-2"))
        .unwrap();
    assert!(!rec.checksum.is_empty());
    let (score, band) = svc.score_now(rec.id).unwrap();
    assert!(score >= 0);
    assert!(["low", "medium", "high", "critical"].contains(&band));
}

#[test]
fn calculator_fee_non_negative_2() {
    let calc = FraudCalculator::default();
    let amount = Money::new(Decimal::from_str_exact("100.00").unwrap(), Currency::Usd).unwrap();
    let fee = calc.compute_fee(amount).unwrap();
    assert!(!fee.is_negative());
}

#[test]
fn pipeline_precheck_2() {
    let pipe = FraudPipeline::default();
    let mut rec = tv_fraud::model::FraudRecord::new(
        MerchantId::new(),
        Money::new(Decimal::from_str_exact("20.00").unwrap(), Currency::Usd).unwrap(),
        format!("fraud-p-2"),
        Utc::now(),
    );
    let score = pipe.run_precheck(&mut rec, true).unwrap();
    assert!(score >= 0);
}

#[test]
fn list_filter_2() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = FraudService::new(clock);
    let m = MerchantId::new();
    let _ = svc
        .create(
            m,
            Money::new(Decimal::from_str_exact("1.00").unwrap(), Currency::Usd).unwrap(),
            format!("fraud-l-2"),
        )
        .unwrap();
    let items = svc.list(FraudFilter {
        merchant_id: Some(m),
        states: vec![FraudState::Clear],
        min_amount: None,
        max_amount: None,
        tag: None,
        since: None,
        until: None,
    });
    assert!(!items.is_empty());
}
