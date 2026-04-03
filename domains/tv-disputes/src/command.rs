use crate::state::DisputesState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum DisputesCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: DisputesState,
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

impl DisputesCommand {
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
            Self::Create { reference, .. } => format!("disputes:create:{reference}:47"),
            Self::Advance { id, next } => format!("disputes:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("disputes:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("disputes:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("disputes:ck:{id}"),
        }
    }
}
