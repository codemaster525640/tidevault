use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComplianceState {
    Pass,
    SoftHold,
    HardHold,
    Escalated,
    Cleared,
}

impl ComplianceState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Pass, Self::SoftHold) => true,
            (Self::SoftHold, Self::HardHold) => true,
            (Self::SoftHold, Self::Cleared) => true,
            (Self::HardHold, Self::Escalated) => true,
            (Self::Escalated, Self::Cleared) => true,
            (Self::HardHold, Self::Cleared) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "compliance cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Escalated | Self::Cleared)
    }
}
