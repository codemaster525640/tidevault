use crate::model::TreasuryRecord;
use crate::state::TreasuryState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &TreasuryRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "treasury",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "treasury",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 101,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(
    rec: &TreasuryRecord,
    from: TreasuryState,
    at: DateTime<Utc>,
) -> EventEnvelope {
    EventEnvelope::wrap(
        "treasury",
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
