use crate::state::CatalogState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum CatalogCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: CatalogState,
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

impl CatalogCommand {
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
            Self::Create { reference, .. } => format!("catalog:create:{reference}:103"),
            Self::Advance { id, next } => format!("catalog:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("catalog:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("catalog:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("catalog:ck:{id}"),
        }
    }
}
