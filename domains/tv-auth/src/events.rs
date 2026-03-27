use crate::model::AuthRecord;
use crate::state::AuthState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &AuthRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "auth",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "auth",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 79,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(rec: &AuthRecord, from: AuthState, at: DateTime<Utc>) -> EventEnvelope {
    EventEnvelope::wrap(
        "auth",
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
