use crate::model::NotifyRecord;
use crate::state::NotifyState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &NotifyRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "notify",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "notify",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 89,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(rec: &NotifyRecord, from: NotifyState, at: DateTime<Utc>) -> EventEnvelope {
    EventEnvelope::wrap(
        "notify",
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
