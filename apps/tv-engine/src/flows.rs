use crate::Engine;
use tv_kernel::clock::Clock;
use tv_kernel::error::Result;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::Money;

pub fn capture_with_compliance<C: Clock + Clone>(
    engine: &mut Engine<C>,
    merchant: MerchantId,
    amount: Money,
    reference: &str,
) -> Result<()> {
    let _ = engine
        .compliance
        .create(merchant, amount, format!("kyc-{reference}"))?;
    let pay = engine.payments.create(merchant, amount, reference)?;
    let _ = engine.payments.soft_progress(pay.id)?;
    let _ = engine
        .webhooks
        .create(merchant, amount, format!("wh-{reference}"))?;
    let _ = engine
        .notify
        .create(merchant, amount, format!("nt-{reference}"))?;
    Ok(())
}

pub fn dispute_lifecycle<C: Clock + Clone>(
    engine: &mut Engine<C>,
    merchant: MerchantId,
    amount: Money,
) -> Result<()> {
    let d = engine.disputes.create(merchant, amount, "disp-1")?;
    let _ = engine.disputes.soft_progress(d.id)?;
    let _ = engine.workflow.create(merchant, amount, "wf-disp")?;
    let _ = engine.audit.create(merchant, amount, "aud-disp")?;
    Ok(())
}

pub fn fx_hedged_payout<C: Clock + Clone>(
    engine: &mut Engine<C>,
    merchant: MerchantId,
    amount: Money,
) -> Result<()> {
    let _ = engine.fx.create(merchant, amount, "fx-quote")?;
    let _ = engine.treasury.create(merchant, amount, "fx-fund")?;
    let _ = engine.rails.create(merchant, amount, "fx-rail")?;
    Ok(())
}
