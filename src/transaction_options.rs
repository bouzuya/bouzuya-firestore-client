use crate::Timestamp;

pub struct TransactionOptions {
    pub max_attempts: Option<usize>,
    pub read_only: Option<bool>,
    pub read_time: Option<Timestamp>,
}
