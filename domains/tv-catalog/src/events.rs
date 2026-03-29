use crate::model::CatalogRecord;
use crate::state::CatalogState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &CatalogRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "catalog",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "catalog",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 103,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(rec: &CatalogRecord, from: CatalogState, at: DateTime<Utc>) -> EventEnvelope {
    EventEnvelope::wrap(
        "catalog",
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
