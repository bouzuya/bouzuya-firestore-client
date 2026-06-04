use crate::Query;
use crate::QueryDocumentSnapshot;
use crate::Timestamp;

/// The result of executing a [`Query`].
///
/// A `QuerySnapshot` carries the documents that matched the query, the
/// [`Query`] that produced them, and the time at which the result was
/// read from the server. Obtain one by calling `get` on a
/// [`CollectionReference`](crate::CollectionReference), a
/// [`CollectionGroup`](crate::CollectionGroup), or any other
/// [`Query`]-shaped value.
///
/// The matching documents are exposed in two ways: iterate the snapshot
/// directly with its [`IntoIterator`] impl to move the documents out
/// without cloning, or call [`docs`](Self::docs) to borrow them as a
/// slice. [`empty`](Self::empty) and [`size`](Self::size) report the
/// cardinality without touching the documents.
///
/// [`Clone`] copies the contained [`QueryDocumentSnapshot`]s, each of
/// which deep-clones its document payload, so cloning a large result set
/// is not free. Prefer iterating once when you can.
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use bouzuya_firestore_client::Firestore;
/// use bouzuya_firestore_client::FirestoreOptions;
///
/// let firestore = Firestore::new(FirestoreOptions::default())?;
/// let query_snapshot = firestore.collection("rooms")?.get().await?;
/// let _n = query_snapshot.size();
/// for query_document_snapshot in query_snapshot {
///     let _id = query_document_snapshot.id();
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct QuerySnapshot {
    query: Query,
    query_document_snapshots: Vec<QueryDocumentSnapshot>,
    read_time: Timestamp,
}

impl QuerySnapshot {
    pub(crate) fn new(
        query: Query,
        query_document_snapshots: Vec<QueryDocumentSnapshot>,
        read_time: Timestamp,
    ) -> Self {
        Self {
            query,
            query_document_snapshots,
            read_time,
        }
    }
}

impl QuerySnapshot {
    /// Returns the result documents as a slice, in the order produced by
    /// the query.
    ///
    /// The returned slice borrows from the snapshot, so no documents are
    /// cloned. When you need to take ownership of the documents instead,
    /// iterate the [`QuerySnapshot`] directly via its [`IntoIterator`]
    /// impl, which moves them out without cloning.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let query_snapshot = firestore.collection("rooms")?.get().await?;
    /// let docs = query_snapshot.docs();
    /// for doc in docs {
    ///     let _id = doc.id();
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn docs(&self) -> &[QueryDocumentSnapshot] {
        &self.query_document_snapshots
    }

    /// Returns `true` if this snapshot contains no documents.
    ///
    /// Equivalent to `self.size() == 0`. A `true` result simply means the
    /// query matched nothing; it is not an error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let query_snapshot = firestore.collection("rooms")?.get().await?;
    /// if query_snapshot.empty() {
    ///     // no results
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn empty(&self) -> bool {
        self.query_document_snapshots.is_empty()
    }

    /// Returns the [`Query`] that produced this snapshot.
    ///
    /// Use the returned value to re-run the query — for example to obtain
    /// a fresher snapshot — without rebuilding it.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let query_snapshot = firestore.collection("rooms")?.get().await?;
    /// let _query = query_snapshot.query();
    /// # Ok(())
    /// # }
    /// ```
    pub fn query(&self) -> Query {
        self.query.clone()
    }

    /// Returns the time at which this query result was read from the
    /// server.
    ///
    /// This describes the read itself, not any particular document, and so
    /// is meaningful even when [`empty`](Self::empty) is `true`. Two
    /// successive reads of the same query yield different read times even
    /// when the matching documents have not changed. Each contained
    /// [`QueryDocumentSnapshot`] also carries its own
    /// [`read_time`](QueryDocumentSnapshot::read_time).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let query_snapshot = firestore.collection("rooms")?.get().await?;
    /// let _read_time = query_snapshot.read_time();
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_time(&self) -> Timestamp {
        self.read_time
    }

    /// Returns the number of documents in this snapshot.
    ///
    /// A return value of `0` is equivalent to [`empty`](Self::empty)
    /// returning `true`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let query_snapshot = firestore.collection("rooms")?.get().await?;
    /// let _n = query_snapshot.size();
    /// # Ok(())
    /// # }
    /// ```
    pub fn size(&self) -> usize {
        self.query_document_snapshots.len()
    }
}

impl IntoIterator for QuerySnapshot {
    type Item = QueryDocumentSnapshot;
    type IntoIter = std::vec::IntoIter<QueryDocumentSnapshot>;

    /// Consumes the snapshot, yielding each [`QueryDocumentSnapshot`] in
    /// query order.
    ///
    /// This is the move-based counterpart to [`docs`](Self::docs): where
    /// [`docs`](Self::docs) borrows the documents as a slice, this hands
    /// them out by value, but the [`QuerySnapshot`] is no longer
    /// available afterwards. If you also need
    /// [`query`](Self::query), [`read_time`](Self::read_time),
    /// [`size`](Self::size), or [`empty`](Self::empty), call them before
    /// iterating — or use [`docs`](Self::docs) to keep the snapshot
    /// around.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let query_snapshot = firestore.collection("rooms")?.get().await?;
    /// for query_document_snapshot in query_snapshot {
    ///     let _id = query_document_snapshot.id();
    /// }
    /// # Ok(())
    /// # }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.query_document_snapshots.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::CollectionReference;
    use crate::Firestore;
    use crate::FirestoreOptions;
    use crate::Query;
    use crate::QuerySnapshot;
    use crate::Timestamp;
    use firestore_path::CollectionPath;
    use std::str::FromStr as _;

    #[tokio::test]
    async fn test_new() -> anyhow::Result<()> {
        let collection_path = CollectionPath::from_str("rooms")?;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let collection_reference = CollectionReference::new(collection_path, firestore);
        let query = Query::collection(collection_reference);
        let _qs = QuerySnapshot::new(query, vec![], Timestamp::now());
        Ok(())
    }
}
