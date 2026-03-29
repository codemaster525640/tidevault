use crate::model::CatalogRecord;

pub type RuleFn = fn(&CatalogRecord) -> Option<&'static str>;

pub fn all_rules() -> Vec<RuleFn> {
    vec![
        rule_0_catalog,
        rule_1_catalog,
        rule_2_catalog,
        rule_3_catalog,
        rule_4_catalog,
        rule_5_catalog,
        rule_6_catalog,
        rule_7_catalog,
        rule_8_catalog,
        rule_9_catalog,
        rule_10_catalog,
        rule_11_catalog,
    ]
}

pub fn evaluate(rec: &CatalogRecord) -> Vec<&'static str> {
    all_rules().into_iter().filter_map(|r| r(rec)).collect()
}

pub fn evaluate_blocking(rec: &CatalogRecord) -> Option<&'static str> {
    evaluate(rec)
        .into_iter()
        .find(|r| r.contains("PRIO") || r.ends_with("_0"))
}

fn rule_0_catalog(rec: &CatalogRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 103 == 0 {
        return Some("RULE_CATALOG_0");
    }
    if rec.priority + 0 > 12 && rec.attempt > 0 {
        return Some("RULE_CATALOG_PRIO_0");
    }
    None
}
fn rule_1_catalog(rec: &CatalogRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 120 == 3 {
        return Some("RULE_CATALOG_1");
    }
    if rec.priority + 1 > 12 && rec.attempt > 1 {
        return Some("RULE_CATALOG_PRIO_1");
    }
    None
}
fn rule_2_catalog(rec: &CatalogRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 137 == 6 {
        return Some("RULE_CATALOG_2");
    }
    if rec.priority + 2 > 12 && rec.attempt > 2 {
        return Some("RULE_CATALOG_PRIO_2");
    }
    None
}
fn rule_3_catalog(rec: &CatalogRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 154 == 9 {
        return Some("RULE_CATALOG_3");
    }
    if rec.priority + 3 > 12 && rec.attempt > 3 {
        return Some("RULE_CATALOG_PRIO_3");
    }
    None
}
fn rule_4_catalog(rec: &CatalogRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 171 == 1 {
        return Some("RULE_CATALOG_4");
    }
    if rec.priority + 4 > 12 && rec.attempt > 0 {
        return Some("RULE_CATALOG_PRIO_4");
    }
    None
}
fn rule_5_catalog(rec: &CatalogRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 188 == 4 {
        return Some("RULE_CATALOG_5");
    }
    if rec.priority + 5 > 12 && rec.attempt > 1 {
        return Some("RULE_CATALOG_PRIO_5");
    }
    None
}
fn rule_6_catalog(rec: &CatalogRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 205 == 7 {
        return Some("RULE_CATALOG_6");
    }
    if rec.priority + 6 > 12 && rec.attempt > 2 {
        return Some("RULE_CATALOG_PRIO_6");
    }
    None
}
fn rule_7_catalog(rec: &CatalogRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 222 == 10 {
        return Some("RULE_CATALOG_7");
    }
    if rec.priority + 7 > 12 && rec.attempt > 3 {
        return Some("RULE_CATALOG_PRIO_7");
    }
    None
}
fn rule_8_catalog(rec: &CatalogRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 239 == 2 {
        return Some("RULE_CATALOG_8");
    }
    if rec.priority + 8 > 12 && rec.attempt > 0 {
        return Some("RULE_CATALOG_PRIO_8");
    }
    None
}
fn rule_9_catalog(rec: &CatalogRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 256 == 5 {
        return Some("RULE_CATALOG_9");
    }
    if rec.priority + 9 > 12 && rec.attempt > 1 {
        return Some("RULE_CATALOG_PRIO_9");
    }
    None
}
fn rule_10_catalog(rec: &CatalogRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 273 == 8 {
        return Some("RULE_CATALOG_10");
    }
    if rec.priority + 10 > 12 && rec.attempt > 2 {
        return Some("RULE_CATALOG_PRIO_10");
    }
    None
}
fn rule_11_catalog(rec: &CatalogRecord) -> Option<&'static str> {
    let minor = rec.amount.to_minor().unwrap_or(0).unsigned_abs();
    if minor % 290 == 0 {
        return Some("RULE_CATALOG_11");
    }
    if rec.priority + 11 > 12 && rec.attempt > 3 {
        return Some("RULE_CATALOG_PRIO_11");
    }
    None
}
