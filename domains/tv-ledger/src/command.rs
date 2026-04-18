use crate::state::LedgerState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum LedgerCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: LedgerState,
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

impl LedgerCommand {
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
            Self::Create { reference, .. } => format!("ledger:create:{reference}:11"),
            Self::Advance { id, next } => format!("ledger:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("ledger:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("ledger:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("ledger:ck:{id}"),
        }
    }
}
