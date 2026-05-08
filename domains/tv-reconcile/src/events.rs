use crate::model::ReconcileRecord;
use crate::state::ReconcileState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &ReconcileRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "reconcile",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "reconcile",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 41,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(
    rec: &ReconcileRecord,
    from: ReconcileState,
    at: DateTime<Utc>,
) -> EventEnvelope {
    EventEnvelope::wrap(
        "reconcile",
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
