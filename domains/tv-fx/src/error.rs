use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FxError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, FxError>;

impl std::fmt::Display for FxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for FxError {}

impl From<tv_kernel::error::KernelError> for FxError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
