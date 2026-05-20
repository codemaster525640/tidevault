use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TreasuryState {
    Forecast,
    Reserved,
    Transferred,
    Reconciled,
}

impl TreasuryState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Forecast, Self::Reserved) => true,
            (Self::Reserved, Self::Transferred) => true,
            (Self::Transferred, Self::Reconciled) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "treasury cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Transferred | Self::Reconciled)
    }
}
