use crate::model::RailsRecord;
use crate::state::RailsState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &RailsRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "rails",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "rails",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 29,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(rec: &RailsRecord, from: RailsState, at: DateTime<Utc>) -> EventEnvelope {
    EventEnvelope::wrap(
        "rails",
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
