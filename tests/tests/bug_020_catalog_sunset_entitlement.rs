use rust_decimal::Decimal;
// BUG020: catalog sunset entitlement
use chrono::Utc;
use tv_engine::Engine;
use tv_kernel::clock::FrozenClock;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::{Currency, Money};
use tv_payments::model::PaymentsFilter;
#[test]
fn bug_020_catalog_sunset_entitlement() {
    let clock = FrozenClock::new(Utc::now());
    let mut engine = Engine::new(clock);
    let merchant = MerchantId::new();
    let amount = Money::new(Decimal::from_str_exact("21.00").unwrap(), Currency::Usd).unwrap();
    let onboard = engine.onboard_merchant_payment(merchant, amount, &format!("bug-020"));
    assert!(
        onboard.is_ok(),
        "BUG020 onboard failed: {:?}",
        onboard.err()
    );
    engine
        .run_settlement_cycle(merchant, amount)
        .expect("settlement");
    let items = engine.payments.list(PaymentsFilter {
        merchant_id: Some(merchant),
        states: vec![],
        min_amount: None,
        max_amount: None,
        tag: None,
        since: None,
        until: None,
    });
    assert!(!items.is_empty(), "BUG020 expected payment records");
    let fee = engine.pricing.fee_quote(amount).unwrap();
    assert!(!fee.is_negative());
}
