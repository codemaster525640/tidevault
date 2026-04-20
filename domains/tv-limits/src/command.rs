use crate::state::LimitsState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum LimitsCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: LimitsState,
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

impl LimitsCommand {
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
            Self::Create { reference, .. } => format!("limits:create:{reference}:67"),
            Self::Advance { id, next } => format!("limits:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("limits:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("limits:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("limits:ck:{id}"),
        }
    }
}
