use crate::model::PricingRecord;

pub type RuleFn = fn(&PricingRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_pricing,
        rule_1_pricing,
        rule_2_pricing,
        rule_3_pricing,
        rule_4_pricing,
        rule_5_pricing,
        rule_6_pricing,
        rule_7_pricing,
        rule_8_pricing,
        rule_9_pricing,
        rule_10_pricing,
        rule_11_pricing,
    ]
}

pub fn evaluate(rec: &PricingRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &PricingRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_pricing(rec: &PricingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 61 == 0 {
        return Some("RULE_PRICING_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_PRICING_PRIO_0");
    }
    None
}
fn rule_1_pricing(rec: &PricingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 78 == 3 {
        return Some("RULE_PRICING_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_PRICING_PRIO_1");
    }
    None
}
fn rule_2_pricing(rec: &PricingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 95 == 6 {
        return Some("RULE_PRICING_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_PRICING_PRIO_2");
    }
    None
}
fn rule_3_pricing(rec: &PricingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 112 == 9 {
        return Some("RULE_PRICING_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_PRICING_PRIO_3");
    }
    None
}
fn rule_4_pricing(rec: &PricingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 129 == 1 {
        return Some("RULE_PRICING_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_PRICING_PRIO_4");
    }
    None
}
fn rule_5_pricing(rec: &PricingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 146 == 4 {
        return Some("RULE_PRICING_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_PRICING_PRIO_5");
    }
    None
}
fn rule_6_pricing(rec: &PricingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 163 == 7 {
        return Some("RULE_PRICING_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_PRICING_PRIO_6");
    }
    None
}
fn rule_7_pricing(rec: &PricingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 180 == 10 {
        return Some("RULE_PRICING_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_PRICING_PRIO_7");
    }
    None
}
fn rule_8_pricing(rec: &PricingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 197 == 2 {
        return Some("RULE_PRICING_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_PRICING_PRIO_8");
    }
    None
}
fn rule_9_pricing(rec: &PricingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 214 == 5 {
        return Some("RULE_PRICING_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_PRICING_PRIO_9");
    }
    None
}
fn rule_10_pricing(rec: &PricingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 231 == 8 {
        return Some("RULE_PRICING_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_PRICING_PRIO_10");
    }
    None
}
fn rule_11_pricing(rec: &PricingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 248 == 0 {
        return Some("RULE_PRICING_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_PRICING_PRIO_11");
    }
    None
}
