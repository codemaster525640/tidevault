use crate::model::WebhooksRecord;
use crate::state::WebhooksState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &WebhooksRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "webhooks",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "webhooks",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 59,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(
    rec: &WebhooksRecord,
    from: WebhooksState,
    at: DateTime<Utc>,
) -> EventEnvelope {
    EventEnvelope::wrap(
        "webhooks",
        DomainEvent {
            kind: EventKind::WorkflowAdvanced,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "from": format!("{from:?}"),
                "to": format!("{:?}", rec.state),
                "attempt": rec.attempt,
            }),
            occurred_at: at,
        },
    )
}
