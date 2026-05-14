use crate::state::SettlementState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum SettlementCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: SettlementState,
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

impl SettlementCommand {
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
            Self::Create { reference, .. } => format!("settlement:create:{reference}:37"),
            Self::Advance { id, next } => format!("settlement:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("settlement:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("settlement:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("settlement:ck:{id}"),
        }
    }
}
