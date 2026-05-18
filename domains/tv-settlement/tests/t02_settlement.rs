use chrono::Utc;
use tv_kernel::clock::FrozenClock;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::{Currency, Money};
use tv_settlement::calc::SettlementCalculator;
use tv_settlement::model::SettlementFilter;
use tv_settlement::pipeline::SettlementPipeline;
use tv_settlement::service::SettlementService;
use tv_settlement::state::SettlementState;
use rust_decimal::Decimal;
#[test]
fn create_validate_and_score_2() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = SettlementService::new(clock);
    let amount = Money::new(Decimal::from_str_exact("12.20").unwrap(), Currency::Usd).unwrap();
    let rec = svc
        .create(MerchantId::new(), amount, format!("settlement-u-2"))
        .unwrap();
    assert!(!rec.checksum.is_empty());
    let (score, band) = svc.score_now(rec.id).unwrap();
    assert!(score >= 0);
    assert!(["low", "medium", "high", "critical"].contains(&band));
}

#[test]
fn calculator_fee_non_negative_2() {
    let calc = SettlementCalculator::default();
    let amount = Money::new(Decimal::from_str_exact("100.00").unwrap(), Currency::Usd).unwrap();
    let fee = calc.compute_fee(amount).unwrap();
    assert!(!fee.is_negative());
}

#[test]
fn pipeline_precheck_2() {
    let pipe = SettlementPipeline::default();
    let mut rec = tv_settlement::model::SettlementRecord::new(
        MerchantId::new(),
        Money::new(Decimal::from_str_exact("20.00").unwrap(), Currency::Usd).unwrap(),
        format!("settlement-p-2"),
        Utc::now(),
    );
    let score = pipe.run_precheck(&mut rec, true).unwrap();
    assert!(score >= 0);
}

#[test]
fn list_filter_2() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = SettlementService::new(clock);
    let m = MerchantId::new();
    let _ = svc
        .create(
            m,
            Money::new(Decimal::from_str_exact("1.00").unwrap(), Currency::Usd).unwrap(),
            format!("settlement-l-2"),
        )
        .unwrap();
    let items = svc.list(SettlementFilter {
        merchant_id: Some(m),
        states: vec![SettlementState::Open],
        min_amount: None,
        max_amount: None,
        tag: None,
        since: None,
        until: None,
    });
    assert!(!items.is_empty());
}
