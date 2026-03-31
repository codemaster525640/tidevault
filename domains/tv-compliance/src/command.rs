use crate::state::ComplianceState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum ComplianceCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: ComplianceState,
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

impl ComplianceCommand {
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
            Self::Create { reference, .. } => format!("compliance:create:{reference}:53"),
            Self::Advance { id, next } => format!("compliance:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("compliance:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("compliance:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("compliance:ck:{id}"),
        }
    }
}
