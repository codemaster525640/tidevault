use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisputesState {
    Opened,
    EvidenceDue,
    Represented,
    Won,
    Lost,
    Expired,
}

impl DisputesState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Opened, Self::EvidenceDue) => true,
            (Self::EvidenceDue, Self::Represented) => true,
            (Self::Represented, Self::Won) => true,
            (Self::Represented, Self::Lost) => true,
            (Self::EvidenceDue, Self::Expired) => true,
            (Self::Opened, Self::Expired) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "disputes cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::Lost | Self::Expired)
    }
}
