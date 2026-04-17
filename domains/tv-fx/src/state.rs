use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FxState {
    Quoted,
    Booked,
    Expired,
    Cancelled,
}

impl FxState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Quoted, Self::Booked) => true,
            (Self::Quoted, Self::Expired) => true,
            (Self::Quoted, Self::Cancelled) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "fx cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Expired | Self::Cancelled)
    }
}
