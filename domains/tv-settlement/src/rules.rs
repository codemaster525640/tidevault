use crate::model::SettlementRecord;

pub type RuleFn = fn(&SettlementRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_settlement,
        rule_1_settlement,
        rule_2_settlement,
        rule_3_settlement,
        rule_4_settlement,
        rule_5_settlement,
        rule_6_settlement,
        rule_7_settlement,
        rule_8_settlement,
        rule_9_settlement,
        rule_10_settlement,
        rule_11_settlement,
    ]
}

pub fn evaluate(rec: &SettlementRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &SettlementRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_settlement(rec: &SettlementRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 37 == 0 {
        return Some("RULE_SETTLEMENT_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_SETTLEMENT_PRIO_0");
    }
    None
}
fn rule_1_settlement(rec: &SettlementRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 54 == 3 {
        return Some("RULE_SETTLEMENT_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_SETTLEMENT_PRIO_1");
    }
    None
}
fn rule_2_settlement(rec: &SettlementRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 71 == 6 {
        return Some("RULE_SETTLEMENT_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_SETTLEMENT_PRIO_2");
    }
    None
}
fn rule_3_settlement(rec: &SettlementRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 88 == 9 {
        return Some("RULE_SETTLEMENT_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_SETTLEMENT_PRIO_3");
    }
    None
}
fn rule_4_settlement(rec: &SettlementRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 105 == 1 {
        return Some("RULE_SETTLEMENT_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_SETTLEMENT_PRIO_4");
    }
    None
}
fn rule_5_settlement(rec: &SettlementRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 122 == 4 {
        return Some("RULE_SETTLEMENT_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_SETTLEMENT_PRIO_5");
    }
    None
}
fn rule_6_settlement(rec: &SettlementRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 139 == 7 {
        return Some("RULE_SETTLEMENT_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_SETTLEMENT_PRIO_6");
    }
    None
}
fn rule_7_settlement(rec: &SettlementRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 156 == 10 {
        return Some("RULE_SETTLEMENT_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_SETTLEMENT_PRIO_7");
    }
    None
}
fn rule_8_settlement(rec: &SettlementRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 173 == 2 {
        return Some("RULE_SETTLEMENT_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_SETTLEMENT_PRIO_8");
    }
    None
}
fn rule_9_settlement(rec: &SettlementRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 190 == 5 {
        return Some("RULE_SETTLEMENT_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_SETTLEMENT_PRIO_9");
    }
    None
}
fn rule_10_settlement(rec: &SettlementRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 207 == 8 {
        return Some("RULE_SETTLEMENT_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_SETTLEMENT_PRIO_10");
    }
    None
}
fn rule_11_settlement(rec: &SettlementRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 224 == 0 {
        return Some("RULE_SETTLEMENT_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_SETTLEMENT_PRIO_11");
    }
    None
}
