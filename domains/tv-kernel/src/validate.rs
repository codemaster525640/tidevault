use crate::error::{KernelError, Result};
use regex_lite_stub::is_email_like;

// Tiny local helper to avoid extra deps; kept in-module for offline builds.
mod regex_lite_stub {
    pub fn is_email_like(s: &str) -> bool {
        let parts: Vec<_> = s.split('@').collect();
        if parts.len() != 2 {
            return false;
        }
        !parts[0].is_empty() && parts[1].contains('.') && !parts[1].starts_with('.')
    }
}

pub fn require_non_empty(field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        Err(KernelError::Validation(format!(
            "{field} must not be empty"
        )))
    } else {
        Ok(())
    }
}

pub fn require_len(field: &str, value: &str, min: usize, max: usize) -> Result<()> {
    let len = value.chars().count();
    if len < min || len > max {
        Err(KernelError::Validation(format!(
            "{field} length {len} outside {min}..={max}"
        )))
    } else {
        Ok(())
    }
}

pub fn require_email(field: &str, value: &str) -> Result<()> {
    if is_email_like(value) {
        Ok(())
    } else {
        Err(KernelError::Validation(format!(
            "{field} is not a valid email"
        )))
    }
}

pub fn require_in_range(field: &str, value: i64, min: i64, max: i64) -> Result<()> {
    if value < min || value > max {
        Err(KernelError::Validation(format!(
            "{field}={value} outside {min}..={max}"
        )))
    } else {
        Ok(())
    }
}

pub fn require_iban_checksum(iban: &str) -> Result<()> {
    let compact: String = iban.chars().filter(|c| !c.is_whitespace()).collect();
    if compact.len() < 15 || compact.len() > 34 {
        return Err(KernelError::Validation("iban length invalid".into()));
    }
    if !compact.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(KernelError::Validation("iban charset invalid".into()));
    }
    // ISO 13616 mod-97 check
    let rearranged = {
        let (a, b) = compact.split_at(4);
        format!("{b}{a}")
    };
    let mut expanded = String::new();
    for c in rearranged.chars() {
        if c.is_ascii_alphabetic() {
            expanded.push_str(&(c.to_ascii_uppercase() as u8 - 55).to_string());
        } else {
            expanded.push(c);
        }
    }
    let mut rem = 0u128;
    for chunk in expanded.as_bytes().chunks(9) {
        let piece = std::str::from_utf8(chunk).unwrap_or("0");
        let n: u128 = format!("{rem}{piece}").parse().unwrap_or(0);
        rem = n % 97;
    }
    if rem != 1 {
        Err(KernelError::Validation("iban checksum failed".into()))
    } else {
        Ok(())
    }
}
