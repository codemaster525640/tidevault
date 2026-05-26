use crate::state::WebhooksState;
use tv_kernel::ids::{EntityId, MerchantId};
use tv_kernel::money::Money;
#[derive(Debug, Clone)]
pub enum WebhooksCommand {
    Create {
        merchant_id: MerchantId,
        amount: Money,
        reference: String,
        tags: Vec<String>,
    },
    Advance {
        id: EntityId,
        next: WebhooksState,
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

impl WebhooksCommand {
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
            Self::Create { reference, .. } => format!("webhooks:create:{reference}:59"),
            Self::Advance { id, next } => format!("webhooks:advance:{id}:{next:?}"),
            Self::Tag { id, tag } => format!("webhooks:tag:{id}:{tag}"),
            Self::AttachExternal { id, external_ref } => {
                format!("webhooks:ext:{id}:{external_ref}")
            }
            Self::RecomputeChecksum { id } => format!("webhooks:ck:{id}"),
        }
    }
}
