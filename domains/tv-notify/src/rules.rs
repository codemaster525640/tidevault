use crate::model::NotifyRecord;

pub type RuleFn = fn(&NotifyRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_notify,
        rule_1_notify,
        rule_2_notify,
        rule_3_notify,
        rule_4_notify,
        rule_5_notify,
        rule_6_notify,
        rule_7_notify,
        rule_8_notify,
        rule_9_notify,
        rule_10_notify,
        rule_11_notify,
    ]
}

pub fn evaluate(rec: &NotifyRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &NotifyRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_notify(rec: &NotifyRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 89 == 0 {
        return Some("RULE_NOTIFY_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_NOTIFY_PRIO_0");
    }
    None
}
fn rule_1_notify(rec: &NotifyRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 106 == 3 {
        return Some("RULE_NOTIFY_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_NOTIFY_PRIO_1");
    }
    None
}
fn rule_2_notify(rec: &NotifyRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 123 == 6 {
        return Some("RULE_NOTIFY_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_NOTIFY_PRIO_2");
    }
    None
}
fn rule_3_notify(rec: &NotifyRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 140 == 9 {
        return Some("RULE_NOTIFY_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_NOTIFY_PRIO_3");
    }
    None
}
fn rule_4_notify(rec: &NotifyRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 157 == 1 {
        return Some("RULE_NOTIFY_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_NOTIFY_PRIO_4");
    }
    None
}
fn rule_5_notify(rec: &NotifyRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 174 == 4 {
        return Some("RULE_NOTIFY_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_NOTIFY_PRIO_5");
    }
    None
}
fn rule_6_notify(rec: &NotifyRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 191 == 7 {
        return Some("RULE_NOTIFY_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_NOTIFY_PRIO_6");
    }
    None
}
fn rule_7_notify(rec: &NotifyRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 208 == 10 {
        return Some("RULE_NOTIFY_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_NOTIFY_PRIO_7");
    }
    None
}
fn rule_8_notify(rec: &NotifyRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 225 == 2 {
        return Some("RULE_NOTIFY_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_NOTIFY_PRIO_8");
    }
    None
}
fn rule_9_notify(rec: &NotifyRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 242 == 5 {
        return Some("RULE_NOTIFY_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_NOTIFY_PRIO_9");
    }
    None
}
fn rule_10_notify(rec: &NotifyRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 259 == 8 {
        return Some("RULE_NOTIFY_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_NOTIFY_PRIO_10");
    }
    None
}
fn rule_11_notify(rec: &NotifyRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 276 == 0 {
        return Some("RULE_NOTIFY_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_NOTIFY_PRIO_11");
    }
    None
}
