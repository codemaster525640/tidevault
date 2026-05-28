use crate::state::WorkflowState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum WorkflowCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: WorkflowState,
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

impl WorkflowCommand {
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
            Self::Create { reference, .. } => format!("workflow:create:{reference}:97"),
            Self::Advance { id, next } => format!("workflow:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("workflow:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("workflow:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("workflow:ck:{id}"),
        }
    }
}
