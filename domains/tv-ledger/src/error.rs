use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, LedgerError>;

impl std::fmt::Display for LedgerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for LedgerError {}

impl From<tv_kernel::error::KernelError> for LedgerError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
