use chrono::Utc;
use tv_kernel::clock::FrozenClock;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::{Currency, Money};
use tv_limits::calc::LimitsCalculator;
use tv_limits::model::LimitsFilter;
use tv_limits::pipeline::LimitsPipeline;
use tv_limits::service::LimitsService;
use tv_limits::state::LimitsState;
use rust_decimal::Decimal;
#[test]
fn create_validate_and_score_1() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = LimitsService::new(clock);
    let amount = Money::new(Decimal::from_str_exact("11.10").unwrap(), Currency::Usd).unwrap();
    let rec = svc
        .create(MerchantId::new(), amount, format!("limits-u-1"))
        .unwrap();
    assert!(!rec.checksum.is_empty());
    let (score, band) = svc.score_now(rec.id).unwrap();
    assert!(score >= 0);
    assert!(["low", "medium", "high", "critical"].contains(&band));
}

#[test]
fn calculator_fee_non_negative_1() {
    let calc = LimitsCalculator::default();
    let amount = Money::new(Decimal::from_str_exact("50.00").unwrap(), Currency::Usd).unwrap();
    let fee = calc.compute_fee(amount).unwrap();
    assert!(!fee.is_negative());
}

#[test]
fn pipeline_precheck_1() {
    let pipe = LimitsPipeline::default();
    let mut rec = tv_limits::model::LimitsRecord::new(
        MerchantId::new(),
        Money::new(Decimal::from_str_exact("20.00").unwrap(), Currency::Usd).unwrap(),
        format!("limits-p-1"),
        Utc::now(),
    );
    let score = pipe.run_precheck(&mut rec, true).unwrap();
    assert!(score >= 0);
}

#[test]
fn list_filter_1() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = LimitsService::new(clock);
    let m = MerchantId::new();
    let _ = svc
        .create(
            m,
            Money::new(Decimal::from_str_exact("1.00").unwrap(), Currency::Usd).unwrap(),
            format!("limits-l-1"),
        )
        .unwrap();
    let items = svc.list(LimitsFilter {
        merchant_id: Some(m),
        states: vec![LimitsState::Ok],
        min_amount: None,
        max_amount: None,
        tag: None,
        since: None,
        until: None,
    });
    assert!(!items.is_empty());
}
