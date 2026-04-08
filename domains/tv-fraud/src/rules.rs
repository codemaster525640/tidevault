use crate::model::FraudRecord;

pub type RuleFn = fn(&FraudRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_fraud,
        rule_1_fraud,
        rule_2_fraud,
        rule_3_fraud,
        rule_4_fraud,
        rule_5_fraud,
        rule_6_fraud,
        rule_7_fraud,
        rule_8_fraud,
        rule_9_fraud,
        rule_10_fraud,
        rule_11_fraud,
    ]
}

pub fn evaluate(rec: &FraudRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &FraudRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_fraud(rec: &FraudRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 43 == 0 {
        return Some("RULE_FRAUD_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_FRAUD_PRIO_0");
    }
    None
}
fn rule_1_fraud(rec: &FraudRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 60 == 3 {
        return Some("RULE_FRAUD_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_FRAUD_PRIO_1");
    }
    None
}
fn rule_2_fraud(rec: &FraudRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 77 == 6 {
        return Some("RULE_FRAUD_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_FRAUD_PRIO_2");
    }
    None
}
fn rule_3_fraud(rec: &FraudRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 94 == 9 {
        return Some("RULE_FRAUD_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_FRAUD_PRIO_3");
    }
    None
}
fn rule_4_fraud(rec: &FraudRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 111 == 1 {
        return Some("RULE_FRAUD_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_FRAUD_PRIO_4");
    }
    None
}
fn rule_5_fraud(rec: &FraudRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 128 == 4 {
        return Some("RULE_FRAUD_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_FRAUD_PRIO_5");
    }
    None
}
fn rule_6_fraud(rec: &FraudRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 145 == 7 {
        return Some("RULE_FRAUD_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_FRAUD_PRIO_6");
    }
    None
}
fn rule_7_fraud(rec: &FraudRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 162 == 10 {
        return Some("RULE_FRAUD_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_FRAUD_PRIO_7");
    }
    None
}
fn rule_8_fraud(rec: &FraudRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 179 == 2 {
        return Some("RULE_FRAUD_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_FRAUD_PRIO_8");
    }
    None
}
fn rule_9_fraud(rec: &FraudRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 196 == 5 {
        return Some("RULE_FRAUD_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_FRAUD_PRIO_9");
    }
    None
}
fn rule_10_fraud(rec: &FraudRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 213 == 8 {
        return Some("RULE_FRAUD_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_FRAUD_PRIO_10");
    }
    None
}
fn rule_11_fraud(rec: &FraudRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 230 == 0 {
        return Some("RULE_FRAUD_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_FRAUD_PRIO_11");
    }
    None
}
