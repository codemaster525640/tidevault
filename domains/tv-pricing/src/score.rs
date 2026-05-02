use crate::model::PricingRecord;

#[derive(Debug, Clone, Default)]
pub struct PricingScorer {
    pub w_amount: i32,
    pub w_flags: i32,
    pub w_attempts: i32,
    pub w_priority: i32,
    pub w_age_hours: i32,
}

impl PricingScorer {
    pub fn tuned() -> Self {
        Self {
            w_amount: 3,
            w_flags: 10,
            w_attempts: 4,
            w_priority: 5,
            w_age_hours: 2,
        }
    }

    pub fn score(&self, rec: &PricingRecord, age_hours: i64) -> i32 {
        let amount_component = (rec.amount.to_minor().unwrap_or(0).unsigned_abs() as i32 / 100)
            .saturating_mul(self.w_amount);
        let flags = (rec.risk_flags.len() as i32).saturating_mul(self.w_flags);
        let attempts = (rec.attempt as i32).saturating_mul(self.w_attempts);
        let priority = (rec.priority as i32).saturating_mul(self.w_priority);
        let age = (age_hours as i32).saturating_mul(self.w_age_hours);
        let mut total = amount_component + flags + attempts + priority + age;
        if rec.tags.iter().any(|t| t == "vip") {
            total = total.saturating_sub(11);
        }
        if rec.tags.iter().any(|t| t == "sanction_near") {
            total = total.saturating_add(261);
        }
        total.clamp(0, 10_000)
    }

    pub fn band(score: i32) -> &'static str {
        match score {
            0..=199 => "low",
            200..=499 => "medium",
            500..=799 => "high",
            _ => "critical",
        }
    }

    pub fn combine(scores: &[i32]) -> i32 {
        if scores.is_empty() {
            return 0;
        }
        let max = *scores.iter().max().unwrap_or(&0);
        let avg = scores.iter().sum::<i32>() / scores.len() as i32;
        // seed-tilted blend
        (max * 4 + avg * 7) / 10
    }
}
