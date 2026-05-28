use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, WorkflowError>;

impl std::fmt::Display for WorkflowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for WorkflowError {}

impl From<tv_kernel::error::KernelError> for WorkflowError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
