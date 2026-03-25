use crate::model::AuditRecord;

pub type RuleFn = fn(&AuditRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_audit,
        rule_1_audit,
        rule_2_audit,
        rule_3_audit,
        rule_4_audit,
        rule_5_audit,
        rule_6_audit,
        rule_7_audit,
        rule_8_audit,
        rule_9_audit,
        rule_10_audit,
        rule_11_audit,
    ]
}

pub fn evaluate(rec: &AuditRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &AuditRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_audit(rec: &AuditRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 83 == 0 {
        return Some("RULE_AUDIT_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_AUDIT_PRIO_0");
    }
    None
}
fn rule_1_audit(rec: &AuditRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 100 == 3 {
        return Some("RULE_AUDIT_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_AUDIT_PRIO_1");
    }
    None
}
fn rule_2_audit(rec: &AuditRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 117 == 6 {
        return Some("RULE_AUDIT_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_AUDIT_PRIO_2");
    }
    None
}
fn rule_3_audit(rec: &AuditRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 134 == 9 {
        return Some("RULE_AUDIT_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_AUDIT_PRIO_3");
    }
    None
}
fn rule_4_audit(rec: &AuditRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 151 == 1 {
        return Some("RULE_AUDIT_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_AUDIT_PRIO_4");
    }
    None
}
fn rule_5_audit(rec: &AuditRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 168 == 4 {
        return Some("RULE_AUDIT_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_AUDIT_PRIO_5");
    }
    None
}
fn rule_6_audit(rec: &AuditRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 185 == 7 {
        return Some("RULE_AUDIT_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_AUDIT_PRIO_6");
    }
    None
}
fn rule_7_audit(rec: &AuditRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 202 == 10 {
        return Some("RULE_AUDIT_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_AUDIT_PRIO_7");
    }
    None
}
fn rule_8_audit(rec: &AuditRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 219 == 2 {
        return Some("RULE_AUDIT_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_AUDIT_PRIO_8");
    }
    None
}
fn rule_9_audit(rec: &AuditRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 236 == 5 {
        return Some("RULE_AUDIT_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_AUDIT_PRIO_9");
    }
    None
}
fn rule_10_audit(rec: &AuditRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 253 == 8 {
        return Some("RULE_AUDIT_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_AUDIT_PRIO_10");
    }
    None
}
fn rule_11_audit(rec: &AuditRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 270 == 0 {
        return Some("RULE_AUDIT_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_AUDIT_PRIO_11");
    }
    None
}
