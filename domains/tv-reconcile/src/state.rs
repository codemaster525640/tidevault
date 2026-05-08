use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReconcileState {
    Unmatched,
    Proposed,
    Matched,
    Broken,
    WrittenOff,
}

impl ReconcileState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Unmatched, Self::Proposed) => true,
            (Self::Proposed, Self::Matched) => true,
            (Self::Proposed, Self::Broken) => true,
            (Self::Broken, Self::Matched) => true,
            (Self::Broken, Self::WrittenOff) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "reconcile cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Broken | Self::WrittenOff)
    }
}
