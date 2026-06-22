use rust_decimal::Decimal;
// BUG056: limit hard breach
use chrono::Utc;
use tv_engine::Engine;
use tv_kernel::clock::FrozenClock;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::{Currency, Money};
use tv_payments::model::PaymentsFilter;
#[test]
fn bug_056_limit_hard_breach() {
    let clock = FrozenClock::new(Utc::now());
    let mut engine = Engine::new(clock);
    let merchant = MerchantId::new();
    let amount = Money::new(Decimal::from_str_exact("7.00").unwrap(), Currency::Usd).unwrap();
    let onboard = engine.onboard_merchant_payment(merchant, amount, &format!("bug-056"));
    assert!(
        onboard.is_ok(),
        "BUG056 onboard failed: {:?}",
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
    assert!(!items.is_empty(), "BUG056 expected payment records");
    let fee = engine.pricing.fee_quote(amount).unwrap();
    assert!(!fee.is_negative());
}
