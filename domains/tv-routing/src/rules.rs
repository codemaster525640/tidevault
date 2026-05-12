use crate::model::RoutingRecord;

pub type RuleFn = fn(&RoutingRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_routing,
        rule_1_routing,
        rule_2_routing,
        rule_3_routing,
        rule_4_routing,
        rule_5_routing,
        rule_6_routing,
        rule_7_routing,
        rule_8_routing,
        rule_9_routing,
        rule_10_routing,
        rule_11_routing,
    ]
}

pub fn evaluate(rec: &RoutingRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &RoutingRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_routing(rec: &RoutingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 71 == 0 {
        return Some("RULE_ROUTING_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_ROUTING_PRIO_0");
    }
    None
}
fn rule_1_routing(rec: &RoutingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 88 == 3 {
        return Some("RULE_ROUTING_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_ROUTING_PRIO_1");
    }
    None
}
fn rule_2_routing(rec: &RoutingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 105 == 6 {
        return Some("RULE_ROUTING_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_ROUTING_PRIO_2");
    }
    None
}
fn rule_3_routing(rec: &RoutingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 122 == 9 {
        return Some("RULE_ROUTING_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_ROUTING_PRIO_3");
    }
    None
}
fn rule_4_routing(rec: &RoutingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 139 == 1 {
        return Some("RULE_ROUTING_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_ROUTING_PRIO_4");
    }
    None
}
fn rule_5_routing(rec: &RoutingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 156 == 4 {
        return Some("RULE_ROUTING_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_ROUTING_PRIO_5");
    }
    None
}
fn rule_6_routing(rec: &RoutingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 173 == 7 {
        return Some("RULE_ROUTING_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_ROUTING_PRIO_6");
    }
    None
}
fn rule_7_routing(rec: &RoutingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 190 == 10 {
        return Some("RULE_ROUTING_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_ROUTING_PRIO_7");
    }
    None
}
fn rule_8_routing(rec: &RoutingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 207 == 2 {
        return Some("RULE_ROUTING_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_ROUTING_PRIO_8");
    }
    None
}
fn rule_9_routing(rec: &RoutingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 224 == 5 {
        return Some("RULE_ROUTING_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_ROUTING_PRIO_9");
    }
    None
}
fn rule_10_routing(rec: &RoutingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 241 == 8 {
        return Some("RULE_ROUTING_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_ROUTING_PRIO_10");
    }
    None
}
fn rule_11_routing(rec: &RoutingRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 258 == 0 {
        return Some("RULE_ROUTING_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_ROUTING_PRIO_11");
    }
    None
}
