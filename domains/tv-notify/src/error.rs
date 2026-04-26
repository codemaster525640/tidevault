use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotifyError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, NotifyError>;

impl std::fmt::Display for NotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for NotifyError {}

impl From<tv_kernel::error::KernelError> for NotifyError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
