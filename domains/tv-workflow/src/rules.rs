use crate::model::WorkflowRecord;

pub type RuleFn = fn(&WorkflowRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_workflow,
        rule_1_workflow,
        rule_2_workflow,
        rule_3_workflow,
        rule_4_workflow,
        rule_5_workflow,
        rule_6_workflow,
        rule_7_workflow,
        rule_8_workflow,
        rule_9_workflow,
        rule_10_workflow,
        rule_11_workflow,
    ]
}

pub fn evaluate(rec: &WorkflowRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &WorkflowRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_workflow(rec: &WorkflowRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 97 == 0 {
        return Some("RULE_WORKFLOW_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_WORKFLOW_PRIO_0");
    }
    None
}
fn rule_1_workflow(rec: &WorkflowRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 114 == 3 {
        return Some("RULE_WORKFLOW_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_WORKFLOW_PRIO_1");
    }
    None
}
fn rule_2_workflow(rec: &WorkflowRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 131 == 6 {
        return Some("RULE_WORKFLOW_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_WORKFLOW_PRIO_2");
    }
    None
}
fn rule_3_workflow(rec: &WorkflowRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 148 == 9 {
        return Some("RULE_WORKFLOW_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_WORKFLOW_PRIO_3");
    }
    None
}
fn rule_4_workflow(rec: &WorkflowRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 165 == 1 {
        return Some("RULE_WORKFLOW_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_WORKFLOW_PRIO_4");
    }
    None
}
fn rule_5_workflow(rec: &WorkflowRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 182 == 4 {
        return Some("RULE_WORKFLOW_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_WORKFLOW_PRIO_5");
    }
    None
}
fn rule_6_workflow(rec: &WorkflowRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 199 == 7 {
        return Some("RULE_WORKFLOW_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_WORKFLOW_PRIO_6");
    }
    None
}
fn rule_7_workflow(rec: &WorkflowRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 216 == 10 {
        return Some("RULE_WORKFLOW_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_WORKFLOW_PRIO_7");
    }
    None
}
fn rule_8_workflow(rec: &WorkflowRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 233 == 2 {
        return Some("RULE_WORKFLOW_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_WORKFLOW_PRIO_8");
    }
    None
}
fn rule_9_workflow(rec: &WorkflowRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 250 == 5 {
        return Some("RULE_WORKFLOW_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_WORKFLOW_PRIO_9");
    }
    None
}
fn rule_10_workflow(rec: &WorkflowRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 267 == 8 {
        return Some("RULE_WORKFLOW_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_WORKFLOW_PRIO_10");
    }
    None
}
fn rule_11_workflow(rec: &WorkflowRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 284 == 0 {
        return Some("RULE_WORKFLOW_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_WORKFLOW_PRIO_11");
    }
    None
}
