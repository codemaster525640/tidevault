use crate::model::LimitsRecord;
use crate::state::LimitsState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &LimitsRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "limits",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "limits",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 67,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(rec: &LimitsRecord, from: LimitsState, at: DateTime<Utc>) -> EventEnvelope {
    EventEnvelope::wrap(
        "limits",
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
