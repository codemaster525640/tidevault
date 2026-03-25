use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuditState {
    Appended,
    Sealed,
    Exported,
}

impl AuditState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Appended, Self::Sealed) => true,
            (Self::Sealed, Self::Exported) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "audit cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Sealed | Self::Exported)
    }
}
