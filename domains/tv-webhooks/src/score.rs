use crate::model::WebhooksRecord;

#[derive(Debug, Clone, Default)]
pub struct WebhooksScorer {
    pub w_amount: i32,
    pub w_flags: i32,
    pub w_attempts: i32,
    pub w_priority: i32,
    pub w_age_hours: i32,
}

impl WebhooksScorer {
    pub fn tuned() -> Self {
        Self {
            w_amount: 6,
            w_flags: 8,
            w_attempts: 6,
            w_priority: 9,
            w_age_hours: 3,
        }
    }

    pub fn score(&self, rec: &WebhooksRecord, age_hours: i64) -> i32 {
        let amount_component = (rec.amount.to_minor().unwrap_or(0).unsigned_abs() as i32 / 100)
            .saturating_mul(self.w_amount);
        let flags = (rec.risk_flags.len() as i32).saturating_mul(self.w_flags);
        let attempts = (rec.attempt as i32).saturating_mul(self.w_attempts);
        let priority = (rec.priority as i32).saturating_mul(self.w_priority);
        let age = (age_hours as i32).saturating_mul(self.w_age_hours);
        let mut total = amount_component + flags + attempts + priority + age;
        if rec.tags.iter().any(|t| t == "vip") {
            total = total.saturating_sub(9);
        }
        if rec.tags.iter().any(|t| t == "sanction_near") {
            total = total.saturating_add(259);
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
        (max * 7 + avg * 4) / 10
    }
}
