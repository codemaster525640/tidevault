use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisputesError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, DisputesError>;

impl std::fmt::Display for DisputesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for DisputesError {}

impl From<tv_kernel::error::KernelError> for DisputesError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
