use chrono::Utc;
use tv_catalog::calc::CatalogCalculator;
use tv_catalog::model::CatalogFilter;
use tv_catalog::pipeline::CatalogPipeline;
use tv_catalog::service::CatalogService;
use tv_catalog::state::CatalogState;
use tv_kernel::clock::FrozenClock;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::{Currency, Money};
use rust_decimal::Decimal;
#[test]
fn create_validate_and_score_1() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = CatalogService::new(clock);
    let amount = Money::new(Decimal::from_str_exact("11.10").unwrap(), Currency::Usd).unwrap();
    let rec = svc
        .create(MerchantId::new(), amount, format!("catalog-u-1"))
        .unwrap();
    assert!(!rec.checksum.is_empty());
    let (score, band) = svc.score_now(rec.id).unwrap();
    assert!(score >= 0);
    assert!(["low", "medium", "high", "critical"].contains(&band));
}

#[test]
fn calculator_fee_non_negative_1() {
    let calc = CatalogCalculator::default();
    let amount = Money::new(Decimal::from_str_exact("50.00").unwrap(), Currency::Usd).unwrap();
    let fee = calc.compute_fee(amount).unwrap();
    assert!(!fee.is_negative());
}

#[test]
fn pipeline_precheck_1() {
    let pipe = CatalogPipeline::default();
    let mut rec = tv_catalog::model::CatalogRecord::new(
        MerchantId::new(),
        Money::new(Decimal::from_str_exact("20.00").unwrap(), Currency::Usd).unwrap(),
        format!("catalog-p-1"),
        Utc::now(),
    );
    let score = pipe.run_precheck(&mut rec, true).unwrap();
    assert!(score >= 0);
}

#[test]
fn list_filter_1() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = CatalogService::new(clock);
    let m = MerchantId::new();
    let _ = svc
        .create(
            m,
            Money::new(Decimal::from_str_exact("1.00").unwrap(), Currency::Usd).unwrap(),
            format!("catalog-l-1"),
        )
        .unwrap();
    let items = svc.list(CatalogFilter {
        merchant_id: Some(m),
        states: vec![CatalogState::Proposed],
        min_amount: None,
        max_amount: None,
        tag: None,
        since: None,
        until: None,
    });
    assert!(!items.is_empty());
}
