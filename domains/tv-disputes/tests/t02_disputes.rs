use chrono::Utc;
use tv_disputes::calc::DisputesCalculator;
use tv_disputes::model::DisputesFilter;
use tv_disputes::pipeline::DisputesPipeline;
use tv_disputes::service::DisputesService;
use tv_disputes::state::DisputesState;
use tv_kernel::clock::FrozenClock;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::{Currency, Money};
use rust_decimal::Decimal;
#[test]
fn create_validate_and_score_2() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = DisputesService::new(clock);
    let amount = Money::new(Decimal::from_str_exact("12.20").unwrap(), Currency::Usd).unwrap();
    let rec = svc
        .create(MerchantId::new(), amount, format!("disputes-u-2"))
        .unwrap();
    assert!(!rec.checksum.is_empty());
    let (score, band) = svc.score_now(rec.id).unwrap();
    assert!(score >= 0);
    assert!(["low", "medium", "high", "critical"].contains(&band));
}

#[test]
fn calculator_fee_non_negative_2() {
    let calc = DisputesCalculator::default();
    let amount = Money::new(Decimal::from_str_exact("100.00").unwrap(), Currency::Usd).unwrap();
    let fee = calc.compute_fee(amount).unwrap();
    assert!(!fee.is_negative());
}

#[test]
fn pipeline_precheck_2() {
    let pipe = DisputesPipeline::default();
    let mut rec = tv_disputes::model::DisputesRecord::new(
        MerchantId::new(),
        Money::new(Decimal::from_str_exact("20.00").unwrap(), Currency::Usd).unwrap(),
        format!("disputes-p-2"),
        Utc::now(),
    );
    let score = pipe.run_precheck(&mut rec, true).unwrap();
    assert!(score >= 0);
}

#[test]
fn list_filter_2() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = DisputesService::new(clock);
    let m = MerchantId::new();
    let _ = svc
        .create(
            m,
            Money::new(Decimal::from_str_exact("1.00").unwrap(), Currency::Usd).unwrap(),
            format!("disputes-l-2"),
        )
        .unwrap();
    let items = svc.list(DisputesFilter {
        merchant_id: Some(m),
        states: vec![DisputesState::Opened],
        min_amount: None,
        max_amount: None,
        tag: None,
        since: None,
        until: None,
    });
    assert!(!items.is_empty());
}
