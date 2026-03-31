use crate::model::ComplianceRecord;
use crate::state::ComplianceState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &ComplianceRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "compliance",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "compliance",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 53,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(
    rec: &ComplianceRecord,
    from: ComplianceState,
    at: DateTime<Utc>,
) -> EventEnvelope {
    EventEnvelope::wrap(
        "compliance",
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
