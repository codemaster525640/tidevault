use crate::state::RailsState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum RailsCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: RailsState,
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

impl RailsCommand {
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
            Self::Create { reference, .. } => format!("rails:create:{reference}:29"),
            Self::Advance { id, next } => format!("rails:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("rails:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("rails:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("rails:ck:{id}"),
        }
    }
}
