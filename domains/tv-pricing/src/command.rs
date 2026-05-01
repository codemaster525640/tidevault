use crate::state::PricingState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum PricingCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: PricingState,
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

impl PricingCommand {
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
            Self::Create { reference, .. } => format!("pricing:create:{reference}:61"),
            Self::Advance { id, next } => format!("pricing:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("pricing:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("pricing:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("pricing:ck:{id}"),
        }
    }
}
