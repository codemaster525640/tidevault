use crate::error::{KernelError, Result};
use rust_decimal::Decimal;

pub fn clamp_decimal(v: Decimal, min: Decimal, max: Decimal) -> Decimal {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

pub fn safe_div(a: Decimal, b: Decimal) -> Result<Decimal> {
    if b.is_zero() {
        Err(KernelError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

pub fn bps_to_ratio(bps: i32) -> Decimal {
    Decimal::new(bps as i64, 4)
}

pub fn pct_to_ratio(pct: Decimal) -> Decimal {
    pct / Decimal::new(100, 0)
}

pub fn weighted_average(values: &[(Decimal, Decimal)]) -> Result<Decimal> {
    let mut num = Decimal::ZERO;
    let mut den = Decimal::ZERO;
    for (v, w) in values {
        num += *v * *w;
        den += *w;
    }
    safe_div(num, den)
}
