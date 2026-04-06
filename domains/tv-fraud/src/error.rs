use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FraudError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, FraudError>;

impl std::fmt::Display for FraudError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for FraudError {}

impl From<tv_kernel::error::KernelError> for FraudError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
