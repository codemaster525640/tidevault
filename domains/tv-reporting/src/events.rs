use crate::model::ReportingRecord;
use crate::state::ReportingState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &ReportingRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "reporting",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "reporting",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 73,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(
    rec: &ReportingRecord,
    from: ReportingState,
    at: DateTime<Utc>,
) -> EventEnvelope {
    EventEnvelope::wrap(
        "reporting",
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
