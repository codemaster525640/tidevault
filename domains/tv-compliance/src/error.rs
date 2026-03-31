use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComplianceError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, ComplianceError>;

impl std::fmt::Display for ComplianceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ComplianceError {}

impl From<tv_kernel::error::KernelError> for ComplianceError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
