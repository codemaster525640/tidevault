use crate::state::ReportingState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum ReportingCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: ReportingState,
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

impl ReportingCommand {
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
            Self::Create { reference, .. } => format!("reporting:create:{reference}:73"),
            Self::Advance { id, next } => format!("reporting:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("reporting:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("reporting:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("reporting:ck:{id}"),
        }
    }
}
