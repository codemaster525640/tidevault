use crate::model::ReportingRecord;

pub type RuleFn = fn(&ReportingRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_reporting,
        rule_1_reporting,
        rule_2_reporting,
        rule_3_reporting,
        rule_4_reporting,
        rule_5_reporting,
        rule_6_reporting,
        rule_7_reporting,
        rule_8_reporting,
        rule_9_reporting,
        rule_10_reporting,
        rule_11_reporting,
    ]
}

pub fn evaluate(rec: &ReportingRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &ReportingRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_reporting(rec: &ReportingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 73 == 0 {
        return Some("RULE_REPORTING_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_REPORTING_PRIO_0");
    }
    None
}
fn rule_1_reporting(rec: &ReportingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 90 == 3 {
        return Some("RULE_REPORTING_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_REPORTING_PRIO_1");
    }
    None
}
fn rule_2_reporting(rec: &ReportingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 107 == 6 {
        return Some("RULE_REPORTING_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_REPORTING_PRIO_2");
    }
    None
}
fn rule_3_reporting(rec: &ReportingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 124 == 9 {
        return Some("RULE_REPORTING_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_REPORTING_PRIO_3");
    }
    None
}
fn rule_4_reporting(rec: &ReportingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 141 == 1 {
        return Some("RULE_REPORTING_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_REPORTING_PRIO_4");
    }
    None
}
fn rule_5_reporting(rec: &ReportingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 158 == 4 {
        return Some("RULE_REPORTING_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_REPORTING_PRIO_5");
    }
    None
}
fn rule_6_reporting(rec: &ReportingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 175 == 7 {
        return Some("RULE_REPORTING_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_REPORTING_PRIO_6");
    }
    None
}
fn rule_7_reporting(rec: &ReportingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 192 == 10 {
        return Some("RULE_REPORTING_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_REPORTING_PRIO_7");
    }
    None
}
fn rule_8_reporting(rec: &ReportingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 209 == 2 {
        return Some("RULE_REPORTING_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_REPORTING_PRIO_8");
    }
    None
}
fn rule_9_reporting(rec: &ReportingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 226 == 5 {
        return Some("RULE_REPORTING_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_REPORTING_PRIO_9");
    }
    None
}
fn rule_10_reporting(rec: &ReportingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 243 == 8 {
        return Some("RULE_REPORTING_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_REPORTING_PRIO_10");
    }
    None
}
fn rule_11_reporting(rec: &ReportingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 260 == 0 {
        return Some("RULE_REPORTING_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_REPORTING_PRIO_11");
    }
    None
}
