use crate::model::DisputesRecord;

pub type RuleFn = fn(&DisputesRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_disputes,
        rule_1_disputes,
        rule_2_disputes,
        rule_3_disputes,
        rule_4_disputes,
        rule_5_disputes,
        rule_6_disputes,
        rule_7_disputes,
        rule_8_disputes,
        rule_9_disputes,
        rule_10_disputes,
        rule_11_disputes,
    ]
}

pub fn evaluate(rec: &DisputesRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &DisputesRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_disputes(rec: &DisputesRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 47 == 0 {
        return Some("RULE_DISPUTES_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_DISPUTES_PRIO_0");
    }
    None
}
fn rule_1_disputes(rec: &DisputesRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 64 == 3 {
        return Some("RULE_DISPUTES_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_DISPUTES_PRIO_1");
    }
    None
}
fn rule_2_disputes(rec: &DisputesRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 81 == 6 {
        return Some("RULE_DISPUTES_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_DISPUTES_PRIO_2");
    }
    None
}
fn rule_3_disputes(rec: &DisputesRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 98 == 9 {
        return Some("RULE_DISPUTES_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_DISPUTES_PRIO_3");
    }
    None
}
fn rule_4_disputes(rec: &DisputesRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 115 == 1 {
        return Some("RULE_DISPUTES_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_DISPUTES_PRIO_4");
    }
    None
}
fn rule_5_disputes(rec: &DisputesRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 132 == 4 {
        return Some("RULE_DISPUTES_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_DISPUTES_PRIO_5");
    }
    None
}
fn rule_6_disputes(rec: &DisputesRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 149 == 7 {
        return Some("RULE_DISPUTES_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_DISPUTES_PRIO_6");
    }
    None
}
fn rule_7_disputes(rec: &DisputesRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 166 == 10 {
        return Some("RULE_DISPUTES_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_DISPUTES_PRIO_7");
    }
    None
}
fn rule_8_disputes(rec: &DisputesRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 183 == 2 {
        return Some("RULE_DISPUTES_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_DISPUTES_PRIO_8");
    }
    None
}
fn rule_9_disputes(rec: &DisputesRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 200 == 5 {
        return Some("RULE_DISPUTES_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_DISPUTES_PRIO_9");
    }
    None
}
fn rule_10_disputes(rec: &DisputesRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 217 == 8 {
        return Some("RULE_DISPUTES_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_DISPUTES_PRIO_10");
    }
    None
}
fn rule_11_disputes(rec: &DisputesRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 234 == 0 {
        return Some("RULE_DISPUTES_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_DISPUTES_PRIO_11");
    }
    None
}
