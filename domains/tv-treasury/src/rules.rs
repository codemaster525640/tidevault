use crate::model::TreasuryRecord;

pub type RuleFn = fn(&TreasuryRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_treasury,
        rule_1_treasury,
        rule_2_treasury,
        rule_3_treasury,
        rule_4_treasury,
        rule_5_treasury,
        rule_6_treasury,
        rule_7_treasury,
        rule_8_treasury,
        rule_9_treasury,
        rule_10_treasury,
        rule_11_treasury,
    ]
}

pub fn evaluate(rec: &TreasuryRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &TreasuryRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_treasury(rec: &TreasuryRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 101 == 0 {
        return Some("RULE_TREASURY_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_TREASURY_PRIO_0");
    }
    None
}
fn rule_1_treasury(rec: &TreasuryRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 118 == 3 {
        return Some("RULE_TREASURY_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_TREASURY_PRIO_1");
    }
    None
}
fn rule_2_treasury(rec: &TreasuryRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 135 == 6 {
        return Some("RULE_TREASURY_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_TREASURY_PRIO_2");
    }
    None
}
fn rule_3_treasury(rec: &TreasuryRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 152 == 9 {
        return Some("RULE_TREASURY_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_TREASURY_PRIO_3");
    }
    None
}
fn rule_4_treasury(rec: &TreasuryRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 169 == 1 {
        return Some("RULE_TREASURY_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_TREASURY_PRIO_4");
    }
    None
}
fn rule_5_treasury(rec: &TreasuryRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 186 == 4 {
        return Some("RULE_TREASURY_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_TREASURY_PRIO_5");
    }
    None
}
fn rule_6_treasury(rec: &TreasuryRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 203 == 7 {
        return Some("RULE_TREASURY_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_TREASURY_PRIO_6");
    }
    None
}
fn rule_7_treasury(rec: &TreasuryRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 220 == 10 {
        return Some("RULE_TREASURY_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_TREASURY_PRIO_7");
    }
    None
}
fn rule_8_treasury(rec: &TreasuryRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 237 == 2 {
        return Some("RULE_TREASURY_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_TREASURY_PRIO_8");
    }
    None
}
fn rule_9_treasury(rec: &TreasuryRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 254 == 5 {
        return Some("RULE_TREASURY_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_TREASURY_PRIO_9");
    }
    None
}
fn rule_10_treasury(rec: &TreasuryRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 271 == 8 {
        return Some("RULE_TREASURY_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_TREASURY_PRIO_10");
    }
    None
}
fn rule_11_treasury(rec: &TreasuryRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 288 == 0 {
        return Some("RULE_TREASURY_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_TREASURY_PRIO_11");
    }
    None
}
