use crate::state::TreasuryState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum TreasuryCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: TreasuryState,
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

impl TreasuryCommand {
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
            Self::Create { reference, .. } => format!("treasury:create:{reference}:101"),
            Self::Advance { id, next } => format!("treasury:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("treasury:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("treasury:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("treasury:ck:{id}"),
        }
    }
}
