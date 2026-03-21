use crate::model::AccountsRecord;
use crate::state::AccountsState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &AccountsRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "accounts",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "accounts",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 17,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(
    rec: &AccountsRecord,
    from: AccountsState,
    at: DateTime<Utc>,
) -> EventEnvelope {
    EventEnvelope::wrap(
        "accounts",
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
