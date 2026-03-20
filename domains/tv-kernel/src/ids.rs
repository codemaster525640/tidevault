use std::fmt;
use uuid::Uuid;

macro_rules! id_type {
    ($name:ident, $prefix:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(pub Uuid);

        impl $name {
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }
            pub fn from_uuid(id: Uuid) -> Self {
                Self(id)
            }
            pub fn as_uuid(&self) -> Uuid {
                self.0
            }
            pub fn prefix() -> &'static str {
                $prefix
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}{}", $prefix, self.0)
            }
        }
    };
}

id_type!(EntityId, "ent_");
id_type!(AccountId, "acc_");
id_type!(MerchantId, "mch_");
id_type!(PaymentId, "pay_");
id_type!(TraceId, "trc_");
id_type!(LedgerEntryId, "led_");
id_type!(SettlementId, "stl_");
id_type!(DisputeId, "dsp_");
id_type!(WebhookId, "whk_");
id_type!(ApiKeyId, "key_");
id_type!(WorkflowId, "wfl_");
id_type!(RailTxnId, "rail_");
id_type!(ReconBreakId, "brk_");
id_type!(InvoiceId, "inv_");
id_type!(CustomerId, "cus_");
