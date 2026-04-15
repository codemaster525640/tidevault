use crate::model::FxRecord;
use crate::state::FxState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &FxRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "fx",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "fx",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 31,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(rec: &FxRecord, from: FxState, at: DateTime<Utc>) -> EventEnvelope {
    EventEnvelope::wrap(
        "fx",
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
