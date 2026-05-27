use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WebhooksState {
    Pending,
    Delivering,
    Delivered,
    DeadLetter,
}

impl WebhooksState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Pending, Self::Delivering) => true,
            (Self::Delivering, Self::Delivered) => true,
            (Self::Delivering, Self::Pending) => true,
            (Self::Delivering, Self::DeadLetter) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "webhooks cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Delivered | Self::DeadLetter)
    }
}
