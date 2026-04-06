use crate::state::FraudState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum FraudCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: FraudState,
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

impl FraudCommand {
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
            Self::Create { reference, .. } => format!("fraud:create:{reference}:43"),
            Self::Advance { id, next } => format!("fraud:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("fraud:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("fraud:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("fraud:ck:{id}"),
        }
    }
}
