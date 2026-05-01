use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PricingError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, PricingError>;

impl std::fmt::Display for PricingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for PricingError {}

impl From<tv_kernel::error::KernelError> for PricingError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
