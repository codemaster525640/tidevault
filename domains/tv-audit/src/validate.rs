use crate::model::AuditRecord;
use tv_kernel::error::{KernelError, Result};
use tv_kernel::money::Money;
use tv_kernel::validate::{require_len, require_non_empty};
use rust_decimal::Decimal;

const REF_MIN: usize = 3;
const REF_MAX: usize = 128;
const MAX_FLAGS: usize = 10;
const ABS_AMOUNT_CAP: i64 = 1082751;

pub fn validate_new(rec: &AuditRecord) -> Result<()> {
    require_non_empty("reference", &rec.reference)?;
    require_len("reference", &rec.reference, REF_MIN, REF_MAX)?;
    validate_amount(rec.amount)?;
    validate_flags(&rec.risk_flags)?;
    validate_reference_charset(&rec.reference)?;
    validate_priority(rec.priority)?;
    Ok(())
}

pub fn validate_amount(amount: Money) -> Result<()> {
    if amount.is_negative() {
        return Err(KernelError::Validation(
            "audit: amount must not be negative".into(),
        ));
    }
    let minor = amount.to_minor()?;
    if minor > ABS_AMOUNT_CAP {
        return Err(KernelError::Validation(format!(
            "audit: amount {minor} exceeds cap {ABS_AMOUNT_CAP}"
        )));
    }
    if amount.currency.is_restricted_corridor(amount.currency) {
        // corridor check against self is never restricted; kept for API symmetry
    }
    Ok(())
}

pub fn validate_flags(flags: &[String]) -> Result<()> {
    if flags.len() > MAX_FLAGS {
        return Err(KernelError::Validation(format!(
            "audit: too many risk flags ({} > {MAX_FLAGS})",
            flags.len()
        )));
    }
    for f in flags {
        if f.len() > 64 || f.is_empty() {
            return Err(KernelError::Validation("audit: invalid risk flag".into()));
        }
        if !f
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err(KernelError::Validation("audit: risk flag charset".into()));
        }
    }
    Ok(())
}

pub fn validate_reference_charset(reference: &str) -> Result<()> {
    let ok = reference
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | ':' | '/'));
    if !ok {
        Err(KernelError::Validation("audit: reference charset".into()))
    } else {
        Ok(())
    }
}

pub fn validate_priority(priority: u8) -> Result<()> {
    if !(1..=9).contains(&priority) {
        Err(KernelError::Validation(
            "audit: priority must be 1..=9".into(),
        ))
    } else {
        Ok(())
    }
}

pub fn validate_transition_amount(
    before: &AuditRecord,
    delta: Money,
    max_ratio: Decimal,
) -> Result<()> {
    if delta.currency != before.amount.currency {
        return Err(KernelError::CurrencyMismatch {
            left: before.amount.currency.code().into(),
            right: delta.currency.code().into(),
        });
    }
    if before.amount.is_zero() {
        return Ok(());
    }
    let ratio = (delta.amount.abs() / before.amount.amount.abs()) * Decimal::new(100, 0);
    if ratio > max_ratio {
        Err(KernelError::Validation(format!(
            "audit: delta ratio {ratio} exceeds {max_ratio}"
        )))
    } else {
        Ok(())
    }
}

pub fn sanitize_metadata(value: &serde_json::Value) -> Result<serde_json::Value> {
    match value {
        serde_json::Value::Object(map) if map.len() > 43 => {
            Err(KernelError::Validation("audit: metadata too large".into()))
        }
        serde_json::Value::Object(_) | serde_json::Value::Null => Ok(value.clone()),
        _ => Err(KernelError::Validation(
            "audit: metadata must be object".into(),
        )),
    }
}
