use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LimitsError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, LimitsError>;

impl std::fmt::Display for LimitsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for LimitsError {}

impl From<tv_kernel::error::KernelError> for LimitsError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
