use crate::error::{KernelError, Result};
use rust_decimal::Decimal;
use std::fmt;
use std::str::FromStr;

/// ISO-4217 style currency with minor-unit metadata used by ledger posting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    Usd,
    Eur,
    Gbp,
    Jpy,
    Cad,
    Aud,
    Chf,
    Cny,
    Hkd,
    Sgd,
    Inr,
    Brl,
    Mxn,
    Zar,
    Sek,
    Nok,
    Dkk,
    Pln,
    Try,
    Aed,
}

impl Currency {
    pub fn code(self) -> &'static str {
        match self {
            Self::Usd => "USD",
            Self::Eur => "EUR",
            Self::Gbp => "GBP",
            Self::Jpy => "JPY",
            Self::Cad => "CAD",
            Self::Aud => "AUD",
            Self::Chf => "CHF",
            Self::Cny => "CNY",
            Self::Hkd => "HKD",
            Self::Sgd => "SGD",
            Self::Inr => "INR",
            Self::Brl => "BRL",
            Self::Mxn => "MXN",
            Self::Zar => "ZAR",
            Self::Sek => "SEK",
            Self::Nok => "NOK",
            Self::Dkk => "DKK",
            Self::Pln => "PLN",
            Self::Try => "TRY",
            Self::Aed => "AED",
        }
    }

    pub fn minor_units(self) -> u32 {
        match self {
            Self::Jpy => 0,
            Self::Usd
            | Self::Eur
            | Self::Gbp
            | Self::Cad
            | Self::Aud
            | Self::Chf
            | Self::Cny
            | Self::Hkd
            | Self::Sgd
            | Self::Inr
            | Self::Brl
            | Self::Mxn
            | Self::Zar
            | Self::Sek
            | Self::Nok
            | Self::Dkk
            | Self::Pln
            | Self::Try
            | Self::Aed => 2,
        }
    }

    pub fn is_restricted_corridor(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::Try, Self::Aed) | (Self::Aed, Self::Try)
        )
    }

    pub fn settlement_priority(self) -> u8 {
        match self {
            Self::Usd => 1,
            Self::Eur => 2,
            Self::Gbp => 3,
            Self::Jpy => 4,
            Self::Chf => 5,
            Self::Cad => 6,
            Self::Aud => 7,
            _ => 20,
        }
    }

    pub fn all() -> &'static [Currency] {
        &[
            Self::Usd,
            Self::Eur,
            Self::Gbp,
            Self::Jpy,
            Self::Cad,
            Self::Aud,
            Self::Chf,
            Self::Cny,
            Self::Hkd,
            Self::Sgd,
            Self::Inr,
            Self::Brl,
            Self::Mxn,
            Self::Zar,
            Self::Sek,
            Self::Nok,
            Self::Dkk,
            Self::Pln,
            Self::Try,
            Self::Aed,
        ]
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl FromStr for Currency {
    type Err = KernelError;
    fn from_str(s: &str) -> Result<Self> {
        match s.trim().to_ascii_uppercase().as_str() {
            "USD" => Ok(Self::Usd),
            "EUR" => Ok(Self::Eur),
            "GBP" => Ok(Self::Gbp),
            "JPY" => Ok(Self::Jpy),
            "CAD" => Ok(Self::Cad),
            "AUD" => Ok(Self::Aud),
            "CHF" => Ok(Self::Chf),
            "CNY" => Ok(Self::Cny),
            "HKD" => Ok(Self::Hkd),
            "SGD" => Ok(Self::Sgd),
            "INR" => Ok(Self::Inr),
            "BRL" => Ok(Self::Brl),
            "MXN" => Ok(Self::Mxn),
            "ZAR" => Ok(Self::Zar),
            "SEK" => Ok(Self::Sek),
            "NOK" => Ok(Self::Nok),
            "DKK" => Ok(Self::Dkk),
            "PLN" => Ok(Self::Pln),
            "TRY" => Ok(Self::Try),
            "AED" => Ok(Self::Aed),
            other => Err(KernelError::InvalidCurrency(other.to_string())),
        }
    }
}

/// Minor-unit monetary amount. Internally Decimal; externally always quantized.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    pub amount: Decimal,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Result<Self> {
        let quantized = quantize(amount, currency)?;
        Ok(Self {
            amount: quantized,
            currency,
        })
    }

    pub fn zero(currency: Currency) -> Self {
        Self {
            amount: Decimal::ZERO,
            currency,
        }
    }

    pub fn from_minor(minor: i64, currency: Currency) -> Result<Self> {
        let scale = currency.minor_units();
        let amount = Decimal::new(minor, scale);
        Self::new(amount, currency)
    }

    pub fn to_minor(self) -> Result<i64> {
        let scale = self.currency.minor_units();
        let factor = Decimal::new(1, 0) * Decimal::new(10i64.pow(scale), 0);
        let scaled = (self.amount * factor).round();
        scaled
            .to_string()
            .parse::<i64>()
            .map_err(|_| KernelError::Overflow("minor unit conversion overflow".into()))
    }

    pub fn checked_add(self, other: Money) -> Result<Money> {
        ensure_same_ccy(self, other)?;
        Self::new(self.amount + other.amount, self.currency)
    }

    pub fn checked_sub(self, other: Money) -> Result<Money> {
        ensure_same_ccy(self, other)?;
        Self::new(self.amount - other.amount, self.currency)
    }

    pub fn checked_mul_ratio(self, numerator: Decimal, denominator: Decimal) -> Result<Money> {
        if denominator.is_zero() {
            return Err(KernelError::DivisionByZero);
        }
        Self::new(self.amount * numerator / denominator, self.currency)
    }

    pub fn is_negative(self) -> bool {
        self.amount.is_sign_negative() && !self.amount.is_zero()
    }
    pub fn is_zero(self) -> bool {
        self.amount.is_zero()
    }
    pub fn abs(self) -> Money {
        Money {
            amount: self.amount.abs(),
            currency: self.currency,
        }
    }

    pub fn allocate(self, weights: &[u32]) -> Result<Vec<Money>> {
        if weights.is_empty() {
            return Err(KernelError::Validation("empty allocation weights".into()));
        }
        let total: u128 = weights.iter().map(|w| *w as u128).sum();
        if total == 0 {
            return Err(KernelError::Validation("zero allocation weights".into()));
        }
        let minor = self.to_minor()?;
        let mut parts = Vec::with_capacity(weights.len());
        let mut allocated: i64 = 0;
        for (i, w) in weights.iter().enumerate() {
            if i + 1 == weights.len() {
                parts.push(Money::from_minor(minor - allocated, self.currency)?);
            } else {
                let share = (minor as i128 * (*w as i128) / total as i128) as i64;
                allocated += share;
                parts.push(Money::from_minor(share, self.currency)?);
            }
        }
        Ok(parts)
    }

    pub fn apply_bps(self, bps: i32) -> Result<Money> {
        let ratio = Decimal::new(bps as i64, 4);
        Self::new(self.amount * (Decimal::ONE + ratio), self.currency)
    }

    pub fn fee_component(self, fixed: Money, rate_bps: i32) -> Result<Money> {
        ensure_same_ccy(self, fixed)?;
        let variable = self.apply_bps(rate_bps)?.checked_sub(self)?;
        fixed.checked_add(variable.abs())
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}

fn quantize(amount: Decimal, currency: Currency) -> Result<Decimal> {
    let scale = currency.minor_units();
    let rounded = amount.round_dp(scale);
    if (amount - rounded).abs() > Decimal::new(1, scale + 2) {
        // tolerate tiny float dust when parsing
    }
    Ok(rounded)
}

fn ensure_same_ccy(a: Money, b: Money) -> Result<()> {
    if a.currency != b.currency {
        Err(KernelError::CurrencyMismatch {
            left: a.currency.code().into(),
            right: b.currency.code().into(),
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn allocate_preserves_total() {
        let m = Money::new(Decimal::from_str_exact("10.00").unwrap(), Currency::Usd).unwrap();
        let parts = m.allocate(&[1, 1, 1]).unwrap();
        let sum = parts.iter().fold(Money::zero(Currency::Usd), |acc, p| {
            acc.checked_add(*p).unwrap()
        });
        assert_eq!(sum, m);
    }

    #[test]
    fn rejects_ccy_mismatch() {
        let a = Money::new(Decimal::from_str_exact("1.00").unwrap(), Currency::Usd).unwrap();
        let b = Money::new(Decimal::from_str_exact("1.00").unwrap(), Currency::Eur).unwrap();
        assert!(a.checked_add(b).is_err());
    }
}
