use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RailsState {
    Queued,
    Submitted,
    Accepted,
    Rejected,
    Settled,
}

impl RailsState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Queued, Self::Submitted) => true,
            (Self::Submitted, Self::Accepted) => true,
            (Self::Submitted, Self::Rejected) => true,
            (Self::Accepted, Self::Settled) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "rails cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Rejected | Self::Settled)
    }
}
