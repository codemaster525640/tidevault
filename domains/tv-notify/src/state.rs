use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotifyState {
    Buffered,
    Sending,
    Sent,
    Suppressed,
    Failed,
}

impl NotifyState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Buffered, Self::Sending) => true,
            (Self::Sending, Self::Sent) => true,
            (Self::Sending, Self::Failed) => true,
            (Self::Buffered, Self::Suppressed) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "notify cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Suppressed | Self::Failed)
    }
}
