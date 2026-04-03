use crate::model::DisputesRecord;
use crate::state::DisputesState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &DisputesRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "disputes",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "disputes",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 47,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(
    rec: &DisputesRecord,
    from: DisputesState,
    at: DateTime<Utc>,
) -> EventEnvelope {
    EventEnvelope::wrap(
        "disputes",
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
