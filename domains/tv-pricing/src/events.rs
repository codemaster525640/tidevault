use crate::model::PricingRecord;
use crate::state::PricingState;
use chrono::{DateTime, Utc};
use tv_kernel::events::{DomainEvent, EventEnvelope, EventKind};

pub fn record_created(rec: &PricingRecord) -> EventEnvelope {
    EventEnvelope::wrap(
        "pricing",
        DomainEvent {
            kind: EventKind::AuditRecorded,
            subject: rec.id.to_string(),
            payload: serde_json::json!({
                "domain": "pricing",
                "state": format!("{:?}", rec.state),
                "amount": rec.amount.to_string(),
                "seed": 61,
            }),
            occurred_at: rec.created_at,
        },
    )
}

pub fn state_changed(rec: &PricingRecord, from: PricingState, at: DateTime<Utc>) -> EventEnvelope {
    EventEnvelope::wrap(
        "pricing",
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
