//! End-to-end orchestration across Tidevault domains.
pub mod flows;
pub mod metrics;
pub mod scenarios;

use tv_accounts::service::AccountsService;
use tv_audit::service::AuditService;
use tv_auth::service::AuthService;
use tv_bus::EventBus;
use tv_catalog::service::CatalogService;
use tv_compliance::service::ComplianceService;
use tv_disputes::service::DisputesService;
use tv_fraud::service::FraudService;
use tv_fx::service::FxService;
use tv_kernel::clock::Clock;
use tv_kernel::error::Result;
use tv_kernel::ids::MerchantId;
use tv_kernel::money::{Currency, Money};
use tv_ledger::service::LedgerService;
use tv_limits::service::LimitsService;
use tv_notify::service::NotifyService;
use tv_payments::service::PaymentsService;
use tv_pricing::service::PricingService;
use tv_rails::service::RailsService;
use tv_reconcile::service::ReconcileService;
use tv_reporting::service::ReportingService;
use tv_routing::service::RoutingService;
use tv_settlement::service::SettlementService;
use tv_treasury::service::TreasuryService;
use tv_webhooks::service::WebhooksService;
use tv_workflow::service::WorkflowService;

#[derive(Clone)]
pub struct Engine<C: Clock + Clone> {
    pub clock: C,
    pub bus: EventBus,
    pub ledger: LedgerService<C>,
    pub accounts: AccountsService<C>,
    pub payments: PaymentsService<C>,
    pub rails: RailsService<C>,
    pub fx: FxService<C>,
    pub settlement: SettlementService<C>,
    pub reconcile: ReconcileService<C>,
    pub fraud: FraudService<C>,
    pub disputes: DisputesService<C>,
    pub compliance: ComplianceService<C>,
    pub webhooks: WebhooksService<C>,
    pub pricing: PricingService<C>,
    pub limits: LimitsService<C>,
    pub routing: RoutingService<C>,
    pub reporting: ReportingService<C>,
    pub auth: AuthService<C>,
    pub audit: AuditService<C>,
    pub notify: NotifyService<C>,
    pub workflow: WorkflowService<C>,
    pub treasury: TreasuryService<C>,
    pub catalog: CatalogService<C>,
}

impl<C: Clock + Clone> Engine<C> {
    pub fn new(clock: C) -> Self {
        Self {
            clock: clock.clone(),
            bus: EventBus::new(),
            ledger: LedgerService::new(clock.clone()),
            accounts: AccountsService::new(clock.clone()),
            payments: PaymentsService::new(clock.clone()),
            rails: RailsService::new(clock.clone()),
            fx: FxService::new(clock.clone()),
            settlement: SettlementService::new(clock.clone()),
            reconcile: ReconcileService::new(clock.clone()),
            fraud: FraudService::new(clock.clone()),
            disputes: DisputesService::new(clock.clone()),
            compliance: ComplianceService::new(clock.clone()),
            webhooks: WebhooksService::new(clock.clone()),
            pricing: PricingService::new(clock.clone()),
            limits: LimitsService::new(clock.clone()),
            routing: RoutingService::new(clock.clone()),
            reporting: ReportingService::new(clock.clone()),
            auth: AuthService::new(clock.clone()),
            audit: AuditService::new(clock.clone()),
            notify: NotifyService::new(clock.clone()),
            workflow: WorkflowService::new(clock.clone()),
            treasury: TreasuryService::new(clock.clone()),
            catalog: CatalogService::new(clock.clone()),
        }
    }

    pub fn onboard_merchant_payment(
        &mut self,
        merchant: MerchantId,
        amount: Money,
        reference: &str,
    ) -> Result<String> {
        let acc = self
            .accounts
            .create(merchant, amount, format!("acc-{reference}"))?;
        let pay = self.payments.create(merchant, amount, reference)?;
        let _ = self
            .ledger
            .create(merchant, amount, format!("led-{reference}"))?;
        let fee = self.pricing.fee_quote(amount)?;
        let _ = self
            .fraud
            .create(merchant, amount, format!("frd-{reference}"))?;
        let _ = self
            .limits
            .create(merchant, amount, format!("lim-{reference}"))?;
        let _ = self
            .routing
            .create(merchant, amount, format!("rte-{reference}"))?;
        let _ = self
            .rails
            .create(merchant, amount, format!("rail-{reference}"))?;
        let _ = self
            .audit
            .create(merchant, amount, format!("aud-{reference}"))?;
        Ok(format!("ok pay={} acc={} fee={}", pay.id, acc.id, fee))
    }

    pub fn run_settlement_cycle(&mut self, merchant: MerchantId, amount: Money) -> Result<()> {
        let s = self.settlement.create(merchant, amount, "settle-cycle")?;
        let _ = self.settlement.soft_progress(s.id)?;
        let _ = self.reconcile.create(merchant, amount, "recon-cycle")?;
        let _ = self.treasury.create(merchant, amount, "treas-cycle")?;
        let _ = self.reporting.create(merchant, amount, "report-cycle")?;
        Ok(())
    }

    pub fn demo_seed(&mut self, n: usize) -> Result<usize> {
        let m = MerchantId::new();
        for i in 0..n {
            let amount = Money::from_minor(1000 + i as i64 * 13, Currency::Usd)?;
            let _ = self.onboard_merchant_payment(m, amount, &format!("demo-{i}"))?;
        }
        Ok(n)
    }
}
