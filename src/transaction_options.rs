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
    /// Whether the transaction is read-only.
    ///
    /// Set to `Some(true)` to indicate a read-only transaction. `Some(false)`
    /// or `None` indicates a read-write transaction.
    ///
    /// [`read_time`](Self::read_time) is only meaningful for read-only
    /// transactions, and [`max_attempts`](Self::max_attempts) is only
    /// meaningful for read-write transactions.
    pub read_only: Option<bool>,
    /// If specified, documents are read at the given time.
    ///
    /// This may not be more than 60 seconds in the past from when the
    /// request is processed by the server.
    ///
    /// Only meaningful for read-only transactions (when
    /// [`read_only`](Self::read_only) is `Some(true)`); ignored for
    /// read-write transactions.
    pub read_time: Option<Timestamp>,
}
