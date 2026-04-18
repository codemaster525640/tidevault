use crate::model::LedgerRecord;
use crate::state::LedgerState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &LedgerRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "ledger",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "ledger",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 11,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(rec: &LedgerRecord, from: LedgerState, at: DateTime<Utc>) -> EventEnvelope {
    EventEnvelope::wrap(
        "ledger",
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
