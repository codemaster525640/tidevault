use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoutingError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, RoutingError>;

impl std::fmt::Display for RoutingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for RoutingError {}

impl From<tv_kernel::error::KernelError> for RoutingError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
