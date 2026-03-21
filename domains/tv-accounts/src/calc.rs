use crate::model::AccountsRecord;
use tv_kernel::error::{KernelError, Result};
use tv_kernel::money::{Currency, Money};
use tv_kernel::quant::{bps_to_ratio, safe_div, weighted_average};
use rust_decimal::Decimal;

/// Domain calculator parameterized for accounts (seed=17).
#[derive(Debug, Clone)]
pub struct AccountsCalculator {
    pub fixed_fee: Money,
    pub rate_bps: i32,
    pub fx_pad_bps: i32,
    pub progressive_thresholds: Vec<(Decimal, i32)>,
}

impl Default for AccountsCalculator {
    fn default() -> Self {
        Self {
            fixed_fee: Money::from_minor(17, Currency::Usd)
                .unwrap_or_else(|_| Money::zero(Currency::Usd)),
            rate_bps: 32,
            fx_pad_bps: 22,
            progressive_thresholds: vec![
                (Decimal::new(117, 0), 13),
                (Decimal::new(1051, 0), 26),
                (Decimal::new(10119, 0), 42),
            ],
        }
    }
}

impl AccountsCalculator {
    pub fn progressive_rate_bps(&self, amount: Money) -> i32 {
        let mut rate = self.rate_bps;
        for (threshold, bps) in &self.progressive_thresholds {
            if amount.amount >= *threshold {
                rate = *bps;
            }
        }
        rate
    }

    pub fn compute_fee(&self, amount: Money) -> Result<Money> {
        let bps = self.progressive_rate_bps(amount);
        let variable = amount.apply_bps(bps)?.checked_sub(amount)?.abs();
        if self.fixed_fee.currency != amount.currency {
            // convert fixed fee notionally by ignoring FX here; caller applies FX module
            return Ok(variable);
        }
        self.fixed_fee.checked_add(variable)
    }

    pub fn net_after_fee(&self, amount: Money) -> Result<Money> {
        let fee = self.compute_fee(amount)?;
        if fee.currency == amount.currency {
            amount.checked_sub(fee)
        } else {
            Ok(amount)
        }
    }

    pub fn exposure_score(&self, rec: &AccountsRecord) -> i32 {
        let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs() as i64;
        let base = (minor / (67)) as i32;
        let priority_boost = rec.priority as i32 * 3;
        let flag_boost = rec.risk_flags.len() as i32 * 18;
        let attempt_penalty = rec.attempt as i32 * 7;
        (base + priority_boost + flag_boost + attempt_penalty).clamp(0, 10_000)
    }

    pub fn split_settlement(&self, amount: Money, legs: usize) -> Result<Vec<Money>> {
        if legs == 0 {
            return Err(KernelError::Validation("legs=0".into()));
        }
        let weights: Vec<u32> = (0..legs).map(|i| ((i * 17 + 3) % 9 + 1) as u32).collect();
        amount.allocate(&weights)
    }

    pub fn blend_rates(&self, observations: &[(Decimal, Decimal)]) -> Result<Decimal> {
        let avg = weighted_average(observations)?;
        let pad = bps_to_ratio(self.fx_pad_bps);
        Ok(avg * (Decimal::ONE + pad))
    }

    pub fn amortization_schedule(
        &self,
        principal: Money,
        periods: u32,
        rate_bps: i32,
    ) -> Result<Vec<Money>> {
        if periods == 0 {
            return Err(KernelError::Validation("periods=0".into()));
        }
        let r = bps_to_ratio(rate_bps) / Decimal::new(periods as i64, 0);
        let mut remaining = principal;
        let mut out = Vec::with_capacity(periods as usize);
        for i in 0..periods {
            if i + 1 == periods {
                out.push(remaining);
                break;
            }
            let interest = Money::new(remaining.amount * r, principal.currency)?;
            let base =
                Money::from_minor(principal.to_minor()? / periods as i64, principal.currency)?;
            let payment = base.checked_add(interest)?;
            remaining = remaining.checked_sub(base)?;
            out.push(payment);
        }
        Ok(out)
    }

    pub fn concentration_index(&self, amounts: &[Money]) -> Result<Decimal> {
        if amounts.is_empty() {
            return Ok(Decimal::ZERO);
        }
        let ccy = amounts[0].currency;
        let mut total = Money::zero(ccy);
        for a in amounts {
            total = total.checked_add(*a)?;
        }
        if total.is_zero() {
            return Ok(Decimal::ZERO);
        }
        let mut hhi = Decimal::ZERO;
        for a in amounts {
            let share = safe_div(a.amount, total.amount)?;
            hhi += share * share;
        }
        Ok(hhi)
    }

    pub fn forecast_curve(&self, start: Money, steps: usize) -> Result<Vec<Money>> {
        let mut cur = start;
        let mut out = Vec::with_capacity(steps);
        let growth = Decimal::new(3 as i64, 3);
        for _ in 0..steps {
            cur = Money::new(cur.amount * (Decimal::ONE + growth), cur.currency)?;
            out.push(cur);
        }
        Ok(out)
    }

    pub fn explain_fee(&self, amount: Money) -> Result<String> {
        let fee = self.compute_fee(amount)?;
        let bps = self.progressive_rate_bps(amount);
        Ok(format!(
            "accounts fee={fee} bps={bps} fixed={} seed=17",
            self.fixed_fee
        ))
    }

    pub fn normalize_score(&self, raw: i32) -> f64 {
        let capped = raw.clamp(0, 10_000) as f64;
        (capped / 10_000.0).powf(1.0 + (2 as f64) / 10.0)
    }

    pub fn expected_shortfall(&self, samples: &[Money], alpha: f64) -> Result<Money> {
        if samples.is_empty() {
            return Err(KernelError::Validation("no samples".into()));
        }
        let ccy = samples[0].currency;
        let mut minors: Vec<i64> = samples.iter().map(|m| m.to_minor().unwrap_or(0)).collect();
        minors.sort_unstable();
        let cutoff = ((1.0 - alpha) * minors.len() as f64).floor() as usize;
        let tail = &minors[cutoff.min(minors.len() - 1)..];
        let avg = tail.iter().sum::<i64>() as f64 / tail.len() as f64;
        Money::from_minor(avg.round() as i64, ccy)
    }
}

pub fn round_half_even_minor(amount: Money) -> Result<Money> {
    let minor = amount.to_minor()?;
    // already quantized; re-encode to enforce invariant
    Money::from_minor(minor, amount.currency)
}

pub fn pct_of(amount: Money, pct: Decimal) -> Result<Money> {
    Money::new(amount.amount * pct / Decimal::new(100, 0), amount.currency)
}

#[allow(dead_code)]
fn _seed_probe() -> i32 {
    // keep seed visible to avoid identical codegen across crates
    17 * 3 + 6 - 7
}
