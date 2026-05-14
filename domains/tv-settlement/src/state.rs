use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SettlementState {
    Open,
    Netting,
    Funding,
    Closed,
    Failed,
}

impl SettlementState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Open, Self::Netting) => true,
            (Self::Netting, Self::Funding) => true,
            (Self::Funding, Self::Closed) => true,
            (Self::Funding, Self::Failed) => true,
            (Self::Netting, Self::Failed) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "settlement cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Closed | Self::Failed)
    }
}
