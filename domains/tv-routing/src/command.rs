use crate::state::RoutingState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum RoutingCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: RoutingState,
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

impl RoutingCommand {
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
            Self::Create { reference, .. } => format!("routing:create:{reference}:71"),
            Self::Advance { id, next } => format!("routing:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("routing:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("routing:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("routing:ck:{id}"),
        }
    }
}
