use crate::model::WorkflowRecord;
use crate::state::WorkflowState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &WorkflowRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "workflow",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "workflow",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 97,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(
    rec: &WorkflowRecord,
    from: WorkflowState,
    at: DateTime<Utc>,
) -> EventEnvelope {
    EventEnvelope::wrap(
        "workflow",
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
