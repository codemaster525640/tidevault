use crate::state::PaymentsState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum PaymentsCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: PaymentsState,
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

impl PaymentsCommand {
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
            Self::Create { reference, .. } => format!("payments:create:{reference}:23"),
            Self::Advance { id, next } => format!("payments:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("payments:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("payments:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("payments:ck:{id}"),
        }
    }
}
