use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebhooksError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, WebhooksError>;

impl std::fmt::Display for WebhooksError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for WebhooksError {}

impl From<tv_kernel::error::KernelError> for WebhooksError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
