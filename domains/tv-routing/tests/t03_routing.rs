use chrono::Utc;
use tv_kernel::clock::FrozenClock;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::{Currency, Money};
use tv_routing::calc::RoutingCalculator;
use tv_routing::model::RoutingFilter;
use tv_routing::pipeline::RoutingPipeline;
use tv_routing::service::RoutingService;
use tv_routing::state::RoutingState;
use rust_decimal::Decimal;
#[test]
fn create_validate_and_score_3() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = RoutingService::new(clock);
    let amount = Money::new(Decimal::from_str_exact("13.30").unwrap(), Currency::Usd).unwrap();
    let rec = svc
        .create(MerchantId::new(), amount, format!("routing-u-3"))
        .unwrap();
    assert!(!rec.checksum.is_empty());
    let (score, band) = svc.score_now(rec.id).unwrap();
    assert!(score >= 0);
    assert!(["low", "medium", "high", "critical"].contains(&band));
}

#[test]
fn calculator_fee_non_negative_3() {
    let calc = RoutingCalculator::default();
    let amount = Money::new(Decimal::from_str_exact("150.00").unwrap(), Currency::Usd).unwrap();
    let fee = calc.compute_fee(amount).unwrap();
    assert!(!fee.is_negative());
}

#[test]
fn pipeline_precheck_3() {
    let pipe = RoutingPipeline::default();
    let mut rec = tv_routing::model::RoutingRecord::new(
        MerchantId::new(),
        Money::new(Decimal::from_str_exact("20.00").unwrap(), Currency::Usd).unwrap(),
        format!("routing-p-3"),
        Utc::now(),
    );
    let score = pipe.run_precheck(&mut rec, true).unwrap();
    assert!(score >= 0);
}

#[test]
fn list_filter_3() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = RoutingService::new(clock);
    let m = MerchantId::new();
    let _ = svc
        .create(
            m,
            Money::new(Decimal::from_str_exact("1.00").unwrap(), Currency::Usd).unwrap(),
            format!("routing-l-3"),
        )
        .unwrap();
    let items = svc.list(RoutingFilter {
        merchant_id: Some(m),
        states: vec![RoutingState::Evaluating],
        min_amount: None,
        max_amount: None,
        tag: None,
        since: None,
        until: None,
    });
    assert!(!items.is_empty());
}
