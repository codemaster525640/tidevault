use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RailsError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, RailsError>;

impl std::fmt::Display for RailsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for RailsError {}

impl From<tv_kernel::error::KernelError> for RailsError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
