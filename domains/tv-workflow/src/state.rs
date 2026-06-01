use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkflowState {
    Started,
    AwaitingApproval,
    Approved,
    Rejected,
    TimedOut,
}

impl WorkflowState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Started, Self::AwaitingApproval) => true,
            (Self::AwaitingApproval, Self::Approved) => true,
            (Self::AwaitingApproval, Self::Rejected) => true,
            (Self::AwaitingApproval, Self::TimedOut) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "workflow cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Rejected | Self::TimedOut)
    }
}
