use chrono::Utc;
use tv_accounts::calc::AccountsCalculator;
use tv_accounts::model::AccountsFilter;
use tv_accounts::pipeline::AccountsPipeline;
use tv_accounts::service::AccountsService;
use tv_accounts::state::AccountsState;
use tv_kernel::clock::FrozenClock;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::{Currency, Money};
use rust_decimal::Decimal;
#[test]
fn create_validate_and_score_3() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = AccountsService::new(clock);
    let amount = Money::new(Decimal::from_str_exact("13.30").unwrap(), Currency::Usd).unwrap();
    let rec = svc
        .create(MerchantId::new(), amount, format!("accounts-u-3"))
        .unwrap();
    assert!(!rec.checksum.is_empty());
    let (score, band) = svc.score_now(rec.id).unwrap();
    assert!(score >= 0);
    assert!(["low", "medium", "high", "critical"].contains(&band));
}

#[test]
fn calculator_fee_non_negative_3() {
    let calc = AccountsCalculator::default();
    let amount = Money::new(Decimal::from_str_exact("150.00").unwrap(), Currency::Usd).unwrap();
    let fee = calc.compute_fee(amount).unwrap();
    assert!(!fee.is_negative());
}

#[test]
fn pipeline_precheck_3() {
    let pipe = AccountsPipeline::default();
    let mut rec = tv_accounts::model::AccountsRecord::new(
        MerchantId::new(),
        Money::new(Decimal::from_str_exact("20.00").unwrap(), Currency::Usd).unwrap(),
        format!("accounts-p-3"),
        Utc::now(),
    );
    let score = pipe.run_precheck(&mut rec, true).unwrap();
    assert!(score >= 0);
}

#[test]
fn list_filter_3() {
    let clock = FrozenClock::new(Utc::now());
    let mut svc = AccountsService::new(clock);
    let m = MerchantId::new();
    let _ = svc
        .create(
            m,
            Money::new(Decimal::from_str_exact("1.00").unwrap(), Currency::Usd).unwrap(),
            format!("accounts-l-3"),
        )
        .unwrap();
    let items = svc.list(AccountsFilter {
        merchant_id: Some(m),
        states: vec![AccountsState::Pending],
        min_amount: None,
        max_amount: None,
        tag: None,
        since: None,
        until: None,
    });
    assert!(!items.is_empty());
}
