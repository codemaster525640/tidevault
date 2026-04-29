use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PaymentsState {
    Created,
    Authorized,
    Captured,
    Refunded,
    Voided,
    Failed,
}

impl PaymentsState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Created, Self::Authorized) => true,
            (Self::Authorized, Self::Captured) => true,
            (Self::Authorized, Self::Voided) => true,
            (Self::Captured, Self::Refunded) => true,
            (Self::Created, Self::Failed) => true,
            (Self::Authorized, Self::Failed) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "payments cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Voided | Self::Failed)
    }
}
