use crate::Query;
use crate::QueryDocumentSnapshot;
use crate::Timestamp;

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
    /// Returns the result documents as a [`Vec`], in the order produced by
    /// the query.
    ///
    /// Each call clones the underlying vector and every contained
    /// [`QueryDocumentSnapshot`], so the cost grows with both the number
    /// of documents and the size of each document. When you only need to
    /// walk the results once, prefer iterating the [`QuerySnapshot`]
    /// directly via its [`IntoIterator`] impl, which moves the documents
    /// out without cloning.
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
    /// for doc in &docs {
    ///     let _id = doc.id();
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn docs(&self) -> Vec<QueryDocumentSnapshot> {
        self.query_document_snapshots.clone()
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
