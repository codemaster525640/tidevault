use crate::model::WebhooksRecord;

pub type RuleFn = fn(&WebhooksRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_webhooks,
        rule_1_webhooks,
        rule_2_webhooks,
        rule_3_webhooks,
        rule_4_webhooks,
        rule_5_webhooks,
        rule_6_webhooks,
        rule_7_webhooks,
        rule_8_webhooks,
        rule_9_webhooks,
        rule_10_webhooks,
        rule_11_webhooks,
    ]
}

pub fn evaluate(rec: &WebhooksRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &WebhooksRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_webhooks(rec: &WebhooksRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 59 == 0 {
        return Some("RULE_WEBHOOKS_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_WEBHOOKS_PRIO_0");
    }
    None
}
fn rule_1_webhooks(rec: &WebhooksRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 76 == 3 {
        return Some("RULE_WEBHOOKS_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_WEBHOOKS_PRIO_1");
    }
    None
}
fn rule_2_webhooks(rec: &WebhooksRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 93 == 6 {
        return Some("RULE_WEBHOOKS_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_WEBHOOKS_PRIO_2");
    }
    None
}
fn rule_3_webhooks(rec: &WebhooksRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 110 == 9 {
        return Some("RULE_WEBHOOKS_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_WEBHOOKS_PRIO_3");
    }
    None
}
fn rule_4_webhooks(rec: &WebhooksRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 127 == 1 {
        return Some("RULE_WEBHOOKS_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_WEBHOOKS_PRIO_4");
    }
    None
}
fn rule_5_webhooks(rec: &WebhooksRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 144 == 4 {
        return Some("RULE_WEBHOOKS_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_WEBHOOKS_PRIO_5");
    }
    None
}
fn rule_6_webhooks(rec: &WebhooksRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 161 == 7 {
        return Some("RULE_WEBHOOKS_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_WEBHOOKS_PRIO_6");
    }
    None
}
fn rule_7_webhooks(rec: &WebhooksRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 178 == 10 {
        return Some("RULE_WEBHOOKS_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_WEBHOOKS_PRIO_7");
    }
    None
}
fn rule_8_webhooks(rec: &WebhooksRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 195 == 2 {
        return Some("RULE_WEBHOOKS_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_WEBHOOKS_PRIO_8");
    }
    None
}
fn rule_9_webhooks(rec: &WebhooksRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 212 == 5 {
        return Some("RULE_WEBHOOKS_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_WEBHOOKS_PRIO_9");
    }
    None
}
fn rule_10_webhooks(rec: &WebhooksRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 229 == 8 {
        return Some("RULE_WEBHOOKS_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_WEBHOOKS_PRIO_10");
    }
    None
}
fn rule_11_webhooks(rec: &WebhooksRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 246 == 0 {
        return Some("RULE_WEBHOOKS_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_WEBHOOKS_PRIO_11");
    }
    None
}
