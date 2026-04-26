use crate::state::NotifyState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum NotifyCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: NotifyState,
    },
    Tag {
        id: EntityId,
        tag: String,
    },
    AttachExternal {
        id: EntityId,
        external_ref: String,
    },
    RecomputeChecksum {
        id: EntityId,
    },
}

impl NotifyCommand {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Create { .. } => "create",
            Self::Advance { .. } => "advance",
            Self::Tag { .. } => "tag",
            Self::AttachExternal { .. } => "attach_external",
            Self::RecomputeChecksum { .. } => "recompute_checksum",
        }
    }

    pub fn idempotency_seed(&self) -> String {
        match self {
            Self::Create { reference, .. } => format!("notify:create:{reference}:89"),
            Self::Advance { id, next } => format!("notify:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("notify:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("notify:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("notify:ck:{id}"),
        }
    }
}
