use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CatalogState {
    Proposed,
    Live,
    Deprecated,
    Sunset,
}

impl CatalogState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Proposed, Self::Live) => true,
            (Self::Live, Self::Deprecated) => true,
            (Self::Deprecated, Self::Sunset) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "catalog cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Deprecated | Self::Sunset)
    }
}
