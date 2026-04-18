use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LedgerState {
    Draft,
    Posted,
    Reversed,
    Locked,
}

impl LedgerState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Draft, Self::Posted) => true,
            (Self::Posted, Self::Reversed) => true,
            (Self::Posted, Self::Locked) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "ledger cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Reversed | Self::Locked)
    }
}
