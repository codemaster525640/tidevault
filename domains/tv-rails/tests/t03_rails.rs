use chrono::Utc;
use tv_kernel::clock::FrozenClock;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::{Currency, Money};
use tv_rails::calc::RailsCalculator;
use tv_rails::model::RailsFilter;
use tv_rails::pipeline::RailsPipeline;
use tv_rails::service::RailsService;
use tv_rails::state::RailsState;
use rust_decimal::Decimal;
#[test]
fn create_validate_and_score_3() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = RailsService::new(clock);
    let amount = Money::new(Decimal::from_str_exact("13.30").unwrap(), Currency::Usd).unwrap();
    let rec = svc
        .create(MerchantId::new(), amount, format!("rails-u-3"))
        .unwrap();
    assert!(!rec.checksum.is_empty());
    let (score, band) = svc.score_now(rec.id).unwrap();
    assert!(score >= 0);
    assert!(["low", "medium", "high", "critical"].contains(&band));
}

#[test]
fn calculator_fee_non_negative_3() {
    let calc = RailsCalculator::default();
    let amount = Money::new(Decimal::from_str_exact("150.00").unwrap(), Currency::Usd).unwrap();
    let fee = calc.compute_fee(amount).unwrap();
    assert!(!fee.is_negative());
}

#[test]
fn pipeline_precheck_3() {
    let pipe = RailsPipeline::default();
    let mut rec = tv_rails::model::RailsRecord::new(
        MerchantId::new(),
        Money::new(Decimal::from_str_exact("20.00").unwrap(), Currency::Usd).unwrap(),
        format!("rails-p-3"),
        Utc::now(),
    );
    let score = pipe.run_precheck(&mut rec, true).unwrap();
    assert!(score >= 0);
}

#[test]
fn list_filter_3() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = RailsService::new(clock);
    let m = MerchantId::new();
    let _ = svc
        .create(
            m,
            Money::new(Decimal::from_str_exact("1.00").unwrap(), Currency::Usd).unwrap(),
            format!("rails-l-3"),
        )
        .unwrap();
    let items = svc.list(RailsFilter {
        merchant_id: Some(m),
        states: vec![RailsState::Queued],
        min_amount: None,
        max_amount: None,
        tag: None,
        since: None,
        until: None,
    });
    assert!(!items.is_empty());
}
