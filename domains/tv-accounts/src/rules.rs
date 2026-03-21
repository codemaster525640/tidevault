use crate::model::AccountsRecord;

pub type RuleFn = fn(&AccountsRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_accounts,
        rule_1_accounts,
        rule_2_accounts,
        rule_3_accounts,
        rule_4_accounts,
        rule_5_accounts,
        rule_6_accounts,
        rule_7_accounts,
        rule_8_accounts,
        rule_9_accounts,
        rule_10_accounts,
        rule_11_accounts,
    ]
}

pub fn evaluate(rec: &AccountsRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &AccountsRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_accounts(rec: &AccountsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 17 == 0 {
        return Some("RULE_ACCOUNTS_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_ACCOUNTS_PRIO_0");
    }
    None
}
fn rule_1_accounts(rec: &AccountsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 34 == 3 {
        return Some("RULE_ACCOUNTS_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_ACCOUNTS_PRIO_1");
    }
    None
}
fn rule_2_accounts(rec: &AccountsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 51 == 6 {
        return Some("RULE_ACCOUNTS_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_ACCOUNTS_PRIO_2");
    }
    None
}
fn rule_3_accounts(rec: &AccountsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 68 == 9 {
        return Some("RULE_ACCOUNTS_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_ACCOUNTS_PRIO_3");
    }
    None
}
fn rule_4_accounts(rec: &AccountsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 85 == 1 {
        return Some("RULE_ACCOUNTS_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_ACCOUNTS_PRIO_4");
    }
    None
}
fn rule_5_accounts(rec: &AccountsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 102 == 4 {
        return Some("RULE_ACCOUNTS_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_ACCOUNTS_PRIO_5");
    }
    None
}
fn rule_6_accounts(rec: &AccountsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 119 == 7 {
        return Some("RULE_ACCOUNTS_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_ACCOUNTS_PRIO_6");
    }
    None
}
fn rule_7_accounts(rec: &AccountsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 136 == 10 {
        return Some("RULE_ACCOUNTS_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_ACCOUNTS_PRIO_7");
    }
    None
}
fn rule_8_accounts(rec: &AccountsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 153 == 2 {
        return Some("RULE_ACCOUNTS_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_ACCOUNTS_PRIO_8");
    }
    None
}
fn rule_9_accounts(rec: &AccountsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 170 == 5 {
        return Some("RULE_ACCOUNTS_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_ACCOUNTS_PRIO_9");
    }
    None
}
fn rule_10_accounts(rec: &AccountsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 187 == 8 {
        return Some("RULE_ACCOUNTS_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_ACCOUNTS_PRIO_10");
    }
    None
}
fn rule_11_accounts(rec: &AccountsRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 204 == 0 {
        return Some("RULE_ACCOUNTS_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_ACCOUNTS_PRIO_11");
    }
    None
}
