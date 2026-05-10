use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReportingState {
    Queued,
    Running,
    Ready,
    Failed,
}

impl ReportingState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Queued, Self::Running) => true,
            (Self::Running, Self::Ready) => true,
            (Self::Running, Self::Failed) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "reporting cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Ready | Self::Failed)
    }
}
