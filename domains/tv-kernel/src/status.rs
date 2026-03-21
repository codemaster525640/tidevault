/// Generic lifecycle used by several domains; domains may embed richer enums.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lifecycle {
    Draft,
    Active,
    Suspended,
    Closed,
    Archived,
}

impl Lifecycle {
    pub fn can_transition(self, next: Self) -> bool {
        use Lifecycle::*;
        matches!(
            (self, next),
            (Draft, Active)
                | (Active, Suspended)
                | (Suspended, Active)
                | (Active, Closed)
                | (Suspended, Closed)
                | (Closed, Archived)
        )
    }
}
