use crate::model::LimitsRecord;

pub type RuleFn = fn(&LimitsRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_limits,
        rule_1_limits,
        rule_2_limits,
        rule_3_limits,
        rule_4_limits,
        rule_5_limits,
        rule_6_limits,
        rule_7_limits,
        rule_8_limits,
        rule_9_limits,
        rule_10_limits,
        rule_11_limits,
    ]
}

pub fn evaluate(rec: &LimitsRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &LimitsRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_limits(rec: &LimitsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 67 == 0 {
        return Some("RULE_LIMITS_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_LIMITS_PRIO_0");
    }
    None
}
fn rule_1_limits(rec: &LimitsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 84 == 3 {
        return Some("RULE_LIMITS_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_LIMITS_PRIO_1");
    }
    None
}
fn rule_2_limits(rec: &LimitsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 101 == 6 {
        return Some("RULE_LIMITS_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_LIMITS_PRIO_2");
    }
    None
}
fn rule_3_limits(rec: &LimitsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 118 == 9 {
        return Some("RULE_LIMITS_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_LIMITS_PRIO_3");
    }
    None
}
fn rule_4_limits(rec: &LimitsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 135 == 1 {
        return Some("RULE_LIMITS_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_LIMITS_PRIO_4");
    }
    None
}
fn rule_5_limits(rec: &LimitsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 152 == 4 {
        return Some("RULE_LIMITS_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_LIMITS_PRIO_5");
    }
    None
}
fn rule_6_limits(rec: &LimitsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 169 == 7 {
        return Some("RULE_LIMITS_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_LIMITS_PRIO_6");
    }
    None
}
fn rule_7_limits(rec: &LimitsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 186 == 10 {
        return Some("RULE_LIMITS_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_LIMITS_PRIO_7");
    }
    None
}
fn rule_8_limits(rec: &LimitsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 203 == 2 {
        return Some("RULE_LIMITS_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_LIMITS_PRIO_8");
    }
    None
}
fn rule_9_limits(rec: &LimitsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 220 == 5 {
        return Some("RULE_LIMITS_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_LIMITS_PRIO_9");
    }
    None
}
fn rule_10_limits(rec: &LimitsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 237 == 8 {
        return Some("RULE_LIMITS_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_LIMITS_PRIO_10");
    }
    None
}
fn rule_11_limits(rec: &LimitsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 254 == 0 {
        return Some("RULE_LIMITS_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_LIMITS_PRIO_11");
    }
    None
}
