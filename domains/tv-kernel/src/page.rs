#[derive(Debug, Clone)]
pub struct PageRequest {
    pub limit: usize,
    pub cursor: Option<String>,
}

impl PageRequest {
    pub fn new(limit: usize) -> Self {
        Self {
            limit: limit.clamp(1, 500),
            cursor: None,
        }
    }

    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
    pub total_hint: Option<u64>,
}

impl<T> Page<T> {
    pub fn from_slice(mut items: Vec<T>, limit: usize) -> Self {
        let next = if items.len() > limit {
            items.truncate(limit);
            Some(format!("offset:{}", limit))
        } else {
            None
        };
        Self {
            items,
            next_cursor: next,
            total_hint: None,
        }
    }
}
