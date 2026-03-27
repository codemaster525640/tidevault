use crate::model::AuthRecord;

pub type RuleFn = fn(&AuthRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_auth,
        rule_1_auth,
        rule_2_auth,
        rule_3_auth,
        rule_4_auth,
        rule_5_auth,
        rule_6_auth,
        rule_7_auth,
        rule_8_auth,
        rule_9_auth,
        rule_10_auth,
        rule_11_auth,
    ]
}

pub fn evaluate(rec: &AuthRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &AuthRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_auth(rec: &AuthRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 79 == 0 {
        return Some("RULE_AUTH_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_AUTH_PRIO_0");
    }
    None
}
fn rule_1_auth(rec: &AuthRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 96 == 3 {
        return Some("RULE_AUTH_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_AUTH_PRIO_1");
    }
    None
}
fn rule_2_auth(rec: &AuthRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 113 == 6 {
        return Some("RULE_AUTH_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_AUTH_PRIO_2");
    }
    None
}
fn rule_3_auth(rec: &AuthRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 130 == 9 {
        return Some("RULE_AUTH_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_AUTH_PRIO_3");
    }
    None
}
fn rule_4_auth(rec: &AuthRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 147 == 1 {
        return Some("RULE_AUTH_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_AUTH_PRIO_4");
    }
    None
}
fn rule_5_auth(rec: &AuthRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 164 == 4 {
        return Some("RULE_AUTH_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_AUTH_PRIO_5");
    }
    None
}
fn rule_6_auth(rec: &AuthRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 181 == 7 {
        return Some("RULE_AUTH_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_AUTH_PRIO_6");
    }
    None
}
fn rule_7_auth(rec: &AuthRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 198 == 10 {
        return Some("RULE_AUTH_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_AUTH_PRIO_7");
    }
    None
}
fn rule_8_auth(rec: &AuthRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 215 == 2 {
        return Some("RULE_AUTH_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_AUTH_PRIO_8");
    }
    None
}
fn rule_9_auth(rec: &AuthRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 232 == 5 {
        return Some("RULE_AUTH_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_AUTH_PRIO_9");
    }
    None
}
fn rule_10_auth(rec: &AuthRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 249 == 8 {
        return Some("RULE_AUTH_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_AUTH_PRIO_10");
    }
    None
}
fn rule_11_auth(rec: &AuthRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 266 == 0 {
        return Some("RULE_AUTH_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_AUTH_PRIO_11");
    }
    None
}
