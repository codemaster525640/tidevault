use crate::model::ComplianceRecord;

pub type RuleFn = fn(&ComplianceRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_compliance,
        rule_1_compliance,
        rule_2_compliance,
        rule_3_compliance,
        rule_4_compliance,
        rule_5_compliance,
        rule_6_compliance,
        rule_7_compliance,
        rule_8_compliance,
        rule_9_compliance,
        rule_10_compliance,
        rule_11_compliance,
    ]
}

pub fn evaluate(rec: &ComplianceRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &ComplianceRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_compliance(rec: &ComplianceRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 53 == 0 {
        return Some("RULE_COMPLIANCE_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_COMPLIANCE_PRIO_0");
    }
    None
}
fn rule_1_compliance(rec: &ComplianceRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 70 == 3 {
        return Some("RULE_COMPLIANCE_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_COMPLIANCE_PRIO_1");
    }
    None
}
fn rule_2_compliance(rec: &ComplianceRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 87 == 6 {
        return Some("RULE_COMPLIANCE_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_COMPLIANCE_PRIO_2");
    }
    None
}
fn rule_3_compliance(rec: &ComplianceRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 104 == 9 {
        return Some("RULE_COMPLIANCE_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_COMPLIANCE_PRIO_3");
    }
    None
}
fn rule_4_compliance(rec: &ComplianceRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 121 == 1 {
        return Some("RULE_COMPLIANCE_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_COMPLIANCE_PRIO_4");
    }
    None
}
fn rule_5_compliance(rec: &ComplianceRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 138 == 4 {
        return Some("RULE_COMPLIANCE_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_COMPLIANCE_PRIO_5");
    }
    None
}
fn rule_6_compliance(rec: &ComplianceRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 155 == 7 {
        return Some("RULE_COMPLIANCE_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_COMPLIANCE_PRIO_6");
    }
    None
}
fn rule_7_compliance(rec: &ComplianceRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 172 == 10 {
        return Some("RULE_COMPLIANCE_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_COMPLIANCE_PRIO_7");
    }
    None
}
fn rule_8_compliance(rec: &ComplianceRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 189 == 2 {
        return Some("RULE_COMPLIANCE_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_COMPLIANCE_PRIO_8");
    }
    None
}
fn rule_9_compliance(rec: &ComplianceRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 206 == 5 {
        return Some("RULE_COMPLIANCE_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_COMPLIANCE_PRIO_9");
    }
    None
}
fn rule_10_compliance(rec: &ComplianceRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 223 == 8 {
        return Some("RULE_COMPLIANCE_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_COMPLIANCE_PRIO_10");
    }
    None
}
fn rule_11_compliance(rec: &ComplianceRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 240 == 0 {
        return Some("RULE_COMPLIANCE_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_COMPLIANCE_PRIO_11");
    }
    None
}
