use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccountsState {
    Pending,
    Active,
    Frozen,
    Closed,
}

impl AccountsState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Pending, Self::Active) => true,
            (Self::Active, Self::Frozen) => true,
            (Self::Frozen, Self::Active) => true,
            (Self::Active, Self::Closed) => true,
            (Self::Frozen, Self::Closed) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "accounts cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Frozen | Self::Closed)
    }
}
