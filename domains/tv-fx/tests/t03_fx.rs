use chrono::Utc;
use tv_fx::calc::FxCalculator;
use tv_fx::model::FxFilter;
use tv_fx::pipeline::FxPipeline;
use tv_fx::service::FxService;
use tv_fx::state::FxState;
use tv_kernel::clock::FrozenClock;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::{Currency, Money};
use rust_decimal::Decimal;
#[test]
fn create_validate_and_score_3() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = FxService::new(clock);
    let amount = Money::new(Decimal::from_str_exact("13.30").unwrap(), Currency::Usd).unwrap();
    let rec = svc
        .create(MerchantId::new(), amount, format!("fx-u-3"))
        .unwrap();
    assert!(!rec.checksum.is_empty());
    let (score, band) = svc.score_now(rec.id).unwrap();
    assert!(score >= 0);
    assert!(["low", "medium", "high", "critical"].contains(&band));
}

#[test]
fn calculator_fee_non_negative_3() {
    let calc = FxCalculator::default();
    let amount = Money::new(Decimal::from_str_exact("150.00").unwrap(), Currency::Usd).unwrap();
    let fee = calc.compute_fee(amount).unwrap();
    assert!(!fee.is_negative());
}

#[test]
fn pipeline_precheck_3() {
    let pipe = FxPipeline::default();
    let mut rec = tv_fx::model::FxRecord::new(
        MerchantId::new(),
        Money::new(Decimal::from_str_exact("20.00").unwrap(), Currency::Usd).unwrap(),
        format!("fx-p-3"),
        Utc::now(),
    );
    let score = pipe.run_precheck(&mut rec, true).unwrap();
    assert!(score >= 0);
}

#[test]
fn list_filter_3() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = FxService::new(clock);
    let m = MerchantId::new();
    let _ = svc
        .create(
            m,
            Money::new(Decimal::from_str_exact("1.00").unwrap(), Currency::Usd).unwrap(),
            format!("fx-l-3"),
        )
        .unwrap();
    let items = svc.list(FxFilter {
        merchant_id: Some(m),
        states: vec![FxState::Quoted],
        min_amount: None,
        max_amount: None,
        tag: None,
        since: None,
        until: None,
    });
    assert!(!items.is_empty());
}
