use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentsError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, PaymentsError>;

impl std::fmt::Display for PaymentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for PaymentsError {}

impl From<tv_kernel::error::KernelError> for PaymentsError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
