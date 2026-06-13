use rust_decimal::Decimal;
// BUG005: settlement partial netting
use chrono::Utc;
use tv_engine::Engine;
use tv_kernel::clock::FrozenClock;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::{Currency, Money};
use tv_payments::model::PaymentsFilter;
#[test]
fn bug_005_settlement_partial_netting() {
    let clock = FrozenClock::new(Utc::now());
    let mut engine = Engine::new(clock);
    let merchant = MerchantId::new();
    let amount = Money::new(Decimal::from_str_exact("6.00").unwrap(), Currency::Usd).unwrap();
    let onboard = engine.onboard_merchant_payment(merchant, amount, &format!("bug-005"));
    assert!(
        onboard.is_ok(),
        "BUG005 onboard failed: {:?}",
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
    assert!(!items.is_empty(), "BUG005 expected payment records");
    let fee = engine.pricing.fee_quote(amount).unwrap();
    assert!(!fee.is_negative());
}
