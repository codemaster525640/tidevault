use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettlementError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, SettlementError>;

impl std::fmt::Display for SettlementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for SettlementError {}

impl From<tv_kernel::error::KernelError> for SettlementError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
