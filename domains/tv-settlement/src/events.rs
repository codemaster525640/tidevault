use crate::model::SettlementRecord;
use crate::state::SettlementState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &SettlementRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "settlement",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "settlement",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 37,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(
    rec: &SettlementRecord,
    from: SettlementState,
    at: DateTime<Utc>,
) -> EventEnvelope {
    EventEnvelope::wrap(
        "settlement",
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
