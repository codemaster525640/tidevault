use crate::state::FxState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum FxCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: FxState,
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

impl FxCommand {
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
            Self::Create { reference, .. } => format!("fx:create:{reference}:31"),
            Self::Advance { id, next } => format!("fx:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("fx:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("fx:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("fx:ck:{id}"),
        }
    }
}
