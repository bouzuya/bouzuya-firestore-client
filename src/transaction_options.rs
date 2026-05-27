use crate::Timestamp;

#[derive(Default)]
pub struct TransactionOptions {
    /// The maximum number of attempts for this transaction. Defaults to `5`
    /// when `None`.
    ///
    /// Only meaningful for read-write transactions (when
    /// [`read_only`](Self::read_only) is `None` or `Some(false)`); read-only
    /// transactions are not retried. Must be at least `1` when set.
    ///
    /// **Note:** This field is currently not honored — retries on contention
    /// are not implemented, and the value is ignored by
    /// [`Firestore::run_transaction`](crate::Firestore::run_transaction).
    pub max_attempts: Option<usize>,
    pub read_only: Option<bool>,
    pub read_time: Option<Timestamp>,
}
