use crate::model::PaymentsRecord;

pub type RuleFn = fn(&PaymentsRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_payments,
        rule_1_payments,
        rule_2_payments,
        rule_3_payments,
        rule_4_payments,
        rule_5_payments,
        rule_6_payments,
        rule_7_payments,
        rule_8_payments,
        rule_9_payments,
        rule_10_payments,
        rule_11_payments,
    ]
}

pub fn evaluate(rec: &PaymentsRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &PaymentsRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_payments(rec: &PaymentsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 23 == 0 {
        return Some("RULE_PAYMENTS_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_PAYMENTS_PRIO_0");
    }
    None
}
fn rule_1_payments(rec: &PaymentsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 40 == 3 {
        return Some("RULE_PAYMENTS_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_PAYMENTS_PRIO_1");
    }
    None
}
fn rule_2_payments(rec: &PaymentsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 57 == 6 {
        return Some("RULE_PAYMENTS_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_PAYMENTS_PRIO_2");
    }
    None
}
fn rule_3_payments(rec: &PaymentsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 74 == 9 {
        return Some("RULE_PAYMENTS_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_PAYMENTS_PRIO_3");
    }
    None
}
fn rule_4_payments(rec: &PaymentsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 91 == 1 {
        return Some("RULE_PAYMENTS_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_PAYMENTS_PRIO_4");
    }
    None
}
fn rule_5_payments(rec: &PaymentsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 108 == 4 {
        return Some("RULE_PAYMENTS_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_PAYMENTS_PRIO_5");
    }
    None
}
fn rule_6_payments(rec: &PaymentsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 125 == 7 {
        return Some("RULE_PAYMENTS_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_PAYMENTS_PRIO_6");
    }
    None
}
fn rule_7_payments(rec: &PaymentsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 142 == 10 {
        return Some("RULE_PAYMENTS_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_PAYMENTS_PRIO_7");
    }
    None
}
fn rule_8_payments(rec: &PaymentsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 159 == 2 {
        return Some("RULE_PAYMENTS_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_PAYMENTS_PRIO_8");
    }
    None
}
fn rule_9_payments(rec: &PaymentsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 176 == 5 {
        return Some("RULE_PAYMENTS_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_PAYMENTS_PRIO_9");
    }
    None
}
fn rule_10_payments(rec: &PaymentsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 193 == 8 {
        return Some("RULE_PAYMENTS_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_PAYMENTS_PRIO_10");
    }
    None
}
fn rule_11_payments(rec: &PaymentsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 210 == 0 {
        return Some("RULE_PAYMENTS_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_PAYMENTS_PRIO_11");
    }
    None
}
