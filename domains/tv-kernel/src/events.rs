use crate::ids::TraceId;
use chrono::{DateTime, Utc};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventKind {
    AccountOpened,
    AccountFrozen,
    PaymentCreated,
    PaymentAuthorized,
    PaymentCaptured,
    PaymentRefunded,
    PaymentFailed,
    LedgerPosted,
    SettlementClosed,
    ReconMatched,
    ReconBreakOpened,
    FraudScoreUpdated,
    DisputeOpened,
    DisputeClosed,
    ComplianceHold,
    ComplianceRelease,
    WebhookDelivered,
    WebhookFailed,
    LimitBreached,
    RouteSelected,
    WorkflowAdvanced,
    AuditRecorded,
    FxRatePublished,
    TreasuryTransfer,
    NotificationQueued,
}

#[derive(Debug, Clone)]
pub struct DomainEvent {
    pub kind: EventKind,
    pub subject: String,
    pub payload: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct EventEnvelope {
    pub id: TraceId,
    pub event: DomainEvent,
    pub causation_id: Option<TraceId>,
    pub correlation_id: Option<TraceId>,
    pub producer: String,
}

impl EventEnvelope {
    pub fn wrap(producer: impl Into<String>, event: DomainEvent) -> Self {
        Self {
            id: TraceId::new(),
            event,
            causation_id: None,
            correlation_id: None,
            producer: producer.into(),
        }
    }

    pub fn with_correlation(mut self, id: TraceId) -> Self {
        self.correlation_id = Some(id);
        self
    }
}
