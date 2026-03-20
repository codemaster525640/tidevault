//! Tidevault kernel: money, identifiers, time, errors, and domain events.
pub mod clock;
pub mod error;
pub mod events;
pub mod hashutil;
pub mod ids;
pub mod money;
pub mod page;
pub mod quant;
pub mod status;
pub mod validate;

pub use clock::{Clock, FrozenClock, SystemClock};
pub use error::{KernelError, Result};
pub use events::{DomainEvent, EventEnvelope, EventKind};
pub use ids::{AccountId, EntityId, MerchantId, PaymentId, TraceId};
pub use money::{Currency, Money};

pub mod algo_1;
pub mod algo_2;
pub mod algo_3;
pub mod algo_4;
pub mod algo_5;
pub mod algo_6;

pub mod history_notes;
