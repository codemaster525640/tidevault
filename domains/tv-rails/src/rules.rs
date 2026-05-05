use crate::model::RailsRecord;

pub type RuleFn = fn(&RailsRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_rails,
        rule_1_rails,
        rule_2_rails,
        rule_3_rails,
        rule_4_rails,
        rule_5_rails,
        rule_6_rails,
        rule_7_rails,
        rule_8_rails,
        rule_9_rails,
        rule_10_rails,
        rule_11_rails,
    ]
}

pub fn evaluate(rec: &RailsRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &RailsRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_rails(rec: &RailsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 29 == 0 {
        return Some("RULE_RAILS_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_RAILS_PRIO_0");
    }
    None
}
fn rule_1_rails(rec: &RailsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 46 == 3 {
        return Some("RULE_RAILS_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_RAILS_PRIO_1");
    }
    None
}
fn rule_2_rails(rec: &RailsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 63 == 6 {
        return Some("RULE_RAILS_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_RAILS_PRIO_2");
    }
    None
}
fn rule_3_rails(rec: &RailsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 80 == 9 {
        return Some("RULE_RAILS_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_RAILS_PRIO_3");
    }
    None
}
fn rule_4_rails(rec: &RailsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 97 == 1 {
        return Some("RULE_RAILS_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_RAILS_PRIO_4");
    }
    None
}
fn rule_5_rails(rec: &RailsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 114 == 4 {
        return Some("RULE_RAILS_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_RAILS_PRIO_5");
    }
    None
}
fn rule_6_rails(rec: &RailsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 131 == 7 {
        return Some("RULE_RAILS_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_RAILS_PRIO_6");
    }
    None
}
fn rule_7_rails(rec: &RailsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 148 == 10 {
        return Some("RULE_RAILS_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_RAILS_PRIO_7");
    }
    None
}
fn rule_8_rails(rec: &RailsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 165 == 2 {
        return Some("RULE_RAILS_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_RAILS_PRIO_8");
    }
    None
}
fn rule_9_rails(rec: &RailsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 182 == 5 {
        return Some("RULE_RAILS_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_RAILS_PRIO_9");
    }
    None
}
fn rule_10_rails(rec: &RailsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 199 == 8 {
        return Some("RULE_RAILS_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_RAILS_PRIO_10");
    }
    None
}
fn rule_11_rails(rec: &RailsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 216 == 0 {
        return Some("RULE_RAILS_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_RAILS_PRIO_11");
    }
    None
}
