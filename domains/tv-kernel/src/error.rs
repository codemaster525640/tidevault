pub type Result<T> = std::result::Result<T, KernelError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelError {
    InvalidCurrency(String),
    CurrencyMismatch { left: String, right: String },
    Validation(String),
    NotFound(String),
    Conflict(String),
    Overflow(String),
    DivisionByZero,
    Unauthorized(String),
    Forbidden(String),
    RateLimited(String),
    Invariant(String),
    Rail(String),
    Timeout(String),
    Internal(String),
}

impl KernelError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::RateLimited(_) | Self::Timeout(_) | Self::Rail(_)
        )
    }

    pub fn http_status_hint(&self) -> u16 {
        match self {
            Self::Validation(_)
            | Self::InvalidCurrency(_)
            | Self::CurrencyMismatch { .. }
            | Self::DivisionByZero => 400,
            Self::Unauthorized(_) => 401,
            Self::Forbidden(_) => 403,
            Self::NotFound(_) => 404,
            Self::Conflict(_) | Self::Invariant(_) => 409,
            Self::RateLimited(_) => 429,
            Self::Timeout(_) => 504,
            _ => 500,
        }
    }
}

impl std::fmt::Display for KernelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for KernelError {}
