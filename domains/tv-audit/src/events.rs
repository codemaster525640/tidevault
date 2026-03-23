use crate::model::AuditRecord;
use crate::state::AuditState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &AuditRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "audit",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "audit",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 83,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(rec: &AuditRecord, from: AuditState, at: DateTime<Utc>) -> EventEnvelope {
    EventEnvelope::wrap(
        "audit",
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
