use tv_kernel::error::KernelError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountsError {
    Kernel(KernelError),
    Domain(String),
}

pub type Result<T> = std::result::Result<T, AccountsError>;

impl std::fmt::Display for AccountsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for AccountsError {}

impl From<tv_kernel::error::KernelError> for AccountsError {
    fn from(value: tv_kernel::error::KernelError) -> Self {
        Self::Kernel(value)
    }
}
