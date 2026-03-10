use crate::Timestamp;

#[derive(Default)]
pub struct TransactionOptions {
    pub max_attempts: Option<usize>,
    pub read_only: Option<bool>,
    pub read_time: Option<Timestamp>,
}
