use crate::model::FraudRecord;
use crate::state::FraudState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &FraudRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "fraud",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "fraud",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 43,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(rec: &FraudRecord, from: FraudState, at: DateTime<Utc>) -> EventEnvelope {
    EventEnvelope::wrap(
        "fraud",
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
