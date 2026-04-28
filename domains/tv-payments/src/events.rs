use crate::model::PaymentsRecord;
use crate::state::PaymentsState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &PaymentsRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "payments",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "payments",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 23,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(
    rec: &PaymentsRecord,
    from: PaymentsState,
    at: DateTime<Utc>,
) -> EventEnvelope {
    EventEnvelope::wrap(
        "payments",
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
