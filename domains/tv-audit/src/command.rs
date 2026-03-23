use crate::state::AuditState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum AuditCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: AuditState,
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

impl AuditCommand {
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
            Self::Create { reference, .. } => format!("audit:create:{reference}:83"),
            Self::Advance { id, next } => format!("audit:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("audit:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("audit:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("audit:ck:{id}"),
        }
    }
}
