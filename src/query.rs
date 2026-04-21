#[allow(clippy::manual_non_exhaustive)]
pub struct Query {
    #[allow(dead_code)]
    limit: Option<i32>,
}

impl Query {
    pub(crate) fn new() -> Self {
        Self { limit: None }
    }
}

impl Query {
    pub fn limit(&self, n: i32) -> Query {
        Query { limit: Some(n) }
    }
}
