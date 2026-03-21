use crate::state::AccountsState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum AccountsCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: AccountsState,
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

impl AccountsCommand {
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
            Self::Create { reference, .. } => format!("accounts:create:{reference}:17"),
            Self::Advance { id, next } => format!("accounts:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("accounts:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("accounts:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("accounts:ck:{id}"),
        }
    }
}
