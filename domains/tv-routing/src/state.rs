use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoutingState {
    Evaluating,
    Selected,
    Cascading,
    Exhausted,
    Committed,
}

impl RoutingState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Evaluating, Self::Selected) => true,
            (Self::Selected, Self::Committed) => true,
            (Self::Selected, Self::Cascading) => true,
            (Self::Cascading, Self::Selected) => true,
            (Self::Cascading, Self::Exhausted) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "routing cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Exhausted | Self::Committed)
    }
}
