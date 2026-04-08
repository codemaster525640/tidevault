use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FraudState {
    Clear,
    Review,
    Blocked,
    Whitelisted,
}

impl FraudState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Clear, Self::Review) => true,
            (Self::Review, Self::Clear) => true,
            (Self::Review, Self::Blocked) => true,
            (Self::Blocked, Self::Whitelisted) => true,
            (Self::Whitelisted, Self::Clear) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "fraud cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Blocked | Self::Whitelisted)
    }
}
