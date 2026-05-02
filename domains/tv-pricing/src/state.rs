use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PricingState {
    Draft,
    Published,
    Retired,
}

impl PricingState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Draft, Self::Published) => true,
            (Self::Published, Self::Retired) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "pricing cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Published | Self::Retired)
    }
}
