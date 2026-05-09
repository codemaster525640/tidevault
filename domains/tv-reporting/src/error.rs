use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportingError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, ReportingError>;

impl std::fmt::Display for ReportingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ReportingError {}

impl From<tv_kernel::error::KernelError> for ReportingError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
