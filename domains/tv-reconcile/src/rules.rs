use crate::model::ReconcileRecord;

pub type RuleFn = fn(&ReconcileRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_reconcile,
        rule_1_reconcile,
        rule_2_reconcile,
        rule_3_reconcile,
        rule_4_reconcile,
        rule_5_reconcile,
        rule_6_reconcile,
        rule_7_reconcile,
        rule_8_reconcile,
        rule_9_reconcile,
        rule_10_reconcile,
        rule_11_reconcile,
    ]
}

pub fn evaluate(rec: &ReconcileRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &ReconcileRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_reconcile(rec: &ReconcileRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 41 == 0 {
        return Some("RULE_RECONCILE_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_RECONCILE_PRIO_0");
    }
    None
}
fn rule_1_reconcile(rec: &ReconcileRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 58 == 3 {
        return Some("RULE_RECONCILE_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_RECONCILE_PRIO_1");
    }
    None
}
fn rule_2_reconcile(rec: &ReconcileRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 75 == 6 {
        return Some("RULE_RECONCILE_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_RECONCILE_PRIO_2");
    }
    None
}
fn rule_3_reconcile(rec: &ReconcileRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 92 == 9 {
        return Some("RULE_RECONCILE_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_RECONCILE_PRIO_3");
    }
    None
}
fn rule_4_reconcile(rec: &ReconcileRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 109 == 1 {
        return Some("RULE_RECONCILE_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_RECONCILE_PRIO_4");
    }
    None
}
fn rule_5_reconcile(rec: &ReconcileRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 126 == 4 {
        return Some("RULE_RECONCILE_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_RECONCILE_PRIO_5");
    }
    None
}
fn rule_6_reconcile(rec: &ReconcileRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 143 == 7 {
        return Some("RULE_RECONCILE_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_RECONCILE_PRIO_6");
    }
    None
}
fn rule_7_reconcile(rec: &ReconcileRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 160 == 10 {
        return Some("RULE_RECONCILE_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_RECONCILE_PRIO_7");
    }
    None
}
fn rule_8_reconcile(rec: &ReconcileRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 177 == 2 {
        return Some("RULE_RECONCILE_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_RECONCILE_PRIO_8");
    }
    None
}
fn rule_9_reconcile(rec: &ReconcileRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 194 == 5 {
        return Some("RULE_RECONCILE_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_RECONCILE_PRIO_9");
    }
    None
}
fn rule_10_reconcile(rec: &ReconcileRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 211 == 8 {
        return Some("RULE_RECONCILE_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_RECONCILE_PRIO_10");
    }
    None
}
fn rule_11_reconcile(rec: &ReconcileRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 228 == 0 {
        return Some("RULE_RECONCILE_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_RECONCILE_PRIO_11");
    }
    None
}
