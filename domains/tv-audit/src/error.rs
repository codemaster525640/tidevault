use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, AuditError>;

impl std::fmt::Display for AuditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for AuditError {}

impl From<tv_kernel::error::KernelError> for AuditError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
