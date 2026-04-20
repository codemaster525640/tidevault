use tv_kernel::error::{KernelError, Result};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LimitsState {
    Ok,
    SoftBreach,
    HardBreach,
    Cooldown,
}

impl LimitsState {
    pub fn can_transition(self, next: Self) -> bool {
        match (self, next) {
            (Self::Ok, Self::SoftBreach) => true,
            (Self::SoftBreach, Self::HardBreach) => true,
            (Self::SoftBreach, Self::Ok) => true,
            (Self::HardBreach, Self::Cooldown) => true,
            (Self::Cooldown, Self::Ok) => true,
            _ => false,
        }
    }

    pub fn transition(self, next: Self) -> Result<Self> {
        if self.can_transition(next) {
            Ok(next)
        } else {
            Err(KernelError::Conflict(format!(
                "limits cannot transition {self:?} -> {next:?}"
            )))
        }
    }

    pub fn terminal(self) -> bool {
        matches!(self, Self::HardBreach | Self::Cooldown)
    }
}
