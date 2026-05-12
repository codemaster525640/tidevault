use crate::model::RoutingRecord;
use crate::state::RoutingState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &RoutingRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "routing",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "routing",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 71,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(rec: &RoutingRecord, from: RoutingState, at: DateTime<Utc>) -> EventEnvelope {
    EventEnvelope::wrap(
        "routing",
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
