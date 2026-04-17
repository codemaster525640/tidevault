use crate::model::FxRecord;

pub type RuleFn = fn(&FxRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_fx, rule_1_fx, rule_2_fx, rule_3_fx, rule_4_fx, rule_5_fx, rule_6_fx, rule_7_fx,
        rule_8_fx, rule_9_fx, rule_10_fx, rule_11_fx,
    ]
}

pub fn evaluate(rec: &FxRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &FxRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_fx(rec: &FxRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 31 == 0 {
        return Some("RULE_FX_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_FX_PRIO_0");
    }
    None
}
fn rule_1_fx(rec: &FxRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 48 == 3 {
        return Some("RULE_FX_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_FX_PRIO_1");
    }
    None
}
fn rule_2_fx(rec: &FxRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 65 == 6 {
        return Some("RULE_FX_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_FX_PRIO_2");
    }
    None
}
fn rule_3_fx(rec: &FxRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 82 == 9 {
        return Some("RULE_FX_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_FX_PRIO_3");
    }
    None
}
fn rule_4_fx(rec: &FxRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 99 == 1 {
        return Some("RULE_FX_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_FX_PRIO_4");
    }
    None
}
fn rule_5_fx(rec: &FxRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 116 == 4 {
        return Some("RULE_FX_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_FX_PRIO_5");
    }
    None
}
fn rule_6_fx(rec: &FxRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 133 == 7 {
        return Some("RULE_FX_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_FX_PRIO_6");
    }
    None
}
fn rule_7_fx(rec: &FxRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 150 == 10 {
        return Some("RULE_FX_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_FX_PRIO_7");
    }
    None
}
fn rule_8_fx(rec: &FxRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 167 == 2 {
        return Some("RULE_FX_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_FX_PRIO_8");
    }
    None
}
fn rule_9_fx(rec: &FxRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 184 == 5 {
        return Some("RULE_FX_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_FX_PRIO_9");
    }
    None
}
fn rule_10_fx(rec: &FxRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 201 == 8 {
        return Some("RULE_FX_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_FX_PRIO_10");
    }
    None
}
fn rule_11_fx(rec: &FxRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 218 == 0 {
        return Some("RULE_FX_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_FX_PRIO_11");
    }
    None
}
