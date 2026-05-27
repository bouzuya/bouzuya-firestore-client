use crate::Timestamp;

/// Options for [`Firestore::run_transaction`](crate::Firestore::run_transaction)
/// to configure a read-write or read-only transaction.
///
/// [`read_only`](Self::read_only) selects the mode: `Some(true)` for
/// read-only, `Some(false)` or `None` for read-write. The remaining fields
/// only apply to one of the two modes — [`read_time`](Self::read_time) to
/// read-only, [`max_attempts`](Self::max_attempts) to read-write — and are
/// ignored in the other.
///
/// [`TransactionOptions::default`] leaves every field `None`, which selects
/// a read-write transaction with default settings.
///
/// # Examples
///
/// ```
/// use bouzuya_firestore_client::TransactionOptions;
///
/// // Read-write transaction with defaults.
/// let _ = TransactionOptions::default();
///
/// // Read-only transaction.
/// let _ = TransactionOptions {
///     read_only: Some(true),
///     ..TransactionOptions::default()
/// };
/// ```
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
