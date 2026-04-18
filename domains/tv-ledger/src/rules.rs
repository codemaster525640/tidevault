use crate::model::LedgerRecord;

pub type RuleFn = fn(&LedgerRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_ledger,
        rule_1_ledger,
        rule_2_ledger,
        rule_3_ledger,
        rule_4_ledger,
        rule_5_ledger,
        rule_6_ledger,
        rule_7_ledger,
        rule_8_ledger,
        rule_9_ledger,
        rule_10_ledger,
        rule_11_ledger,
    ]
}

pub fn evaluate(rec: &LedgerRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &LedgerRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_ledger(rec: &LedgerRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 11 == 0 {
        return Some("RULE_LEDGER_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_LEDGER_PRIO_0");
    }
    None
}
fn rule_1_ledger(rec: &LedgerRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 28 == 3 {
        return Some("RULE_LEDGER_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_LEDGER_PRIO_1");
    }
    None
}
fn rule_2_ledger(rec: &LedgerRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 45 == 6 {
        return Some("RULE_LEDGER_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_LEDGER_PRIO_2");
    }
    None
}
fn rule_3_ledger(rec: &LedgerRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 62 == 9 {
        return Some("RULE_LEDGER_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_LEDGER_PRIO_3");
    }
    None
}
fn rule_4_ledger(rec: &LedgerRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 79 == 1 {
        return Some("RULE_LEDGER_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_LEDGER_PRIO_4");
    }
    None
}
fn rule_5_ledger(rec: &LedgerRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 96 == 4 {
        return Some("RULE_LEDGER_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_LEDGER_PRIO_5");
    }
    None
}
fn rule_6_ledger(rec: &LedgerRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 113 == 7 {
        return Some("RULE_LEDGER_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_LEDGER_PRIO_6");
    }
    None
}
fn rule_7_ledger(rec: &LedgerRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 130 == 10 {
        return Some("RULE_LEDGER_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_LEDGER_PRIO_7");
    }
    None
}
fn rule_8_ledger(rec: &LedgerRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 147 == 2 {
        return Some("RULE_LEDGER_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_LEDGER_PRIO_8");
    }
    None
}
fn rule_9_ledger(rec: &LedgerRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 164 == 5 {
        return Some("RULE_LEDGER_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_LEDGER_PRIO_9");
    }
    None
}
fn rule_10_ledger(rec: &LedgerRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 181 == 8 {
        return Some("RULE_LEDGER_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_LEDGER_PRIO_10");
    }
    None
}
fn rule_11_ledger(rec: &LedgerRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 198 == 0 {
        return Some("RULE_LEDGER_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_LEDGER_PRIO_11");
    }
    None
}
