use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, CatalogError>;

impl std::fmt::Display for CatalogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for CatalogError {}

impl From<tv_kernel::error::KernelError> for CatalogError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
