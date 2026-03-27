use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, AuthError>;

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for AuthError {}

impl From<tv_kernel::error::KernelError> for AuthError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
