use crate::DocumentReference;
use crate::DocumentSnapshot;
use crate::Error;
use crate::Timestamp;

#[derive(Clone)]
pub struct QueryDocumentSnapshot(DocumentSnapshot);

impl QueryDocumentSnapshot {
    pub(crate) fn new(document_snapshot: DocumentSnapshot) -> Self {
        assert!(document_snapshot.exists());
        Self(document_snapshot)
    }
}

impl QueryDocumentSnapshot {
    /// Returns the server-side time at which this document was created.
    ///
    /// A [`QueryDocumentSnapshot`] always represents a document that
    /// exists, so unlike [`DocumentSnapshot::create_time`] this returns a
    /// [`Timestamp`] directly rather than an [`Option`]. The returned
    /// timestamp is the commit time of the original
    /// [`DocumentReference::create`] or [`DocumentReference::set`] that
    /// brought the document into being, not the time of the most recent
    /// write — use [`update_time`](Self::update_time) for that.
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
    ///     let _create_time = query_document_snapshot.create_time();
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_time(&self) -> Timestamp {
        self.0.create_time().expect("document exists")
    }

    /// Deserializes this document's fields into `T`.
    ///
    /// A [`QueryDocumentSnapshot`] always represents a document that
    /// exists, so unlike [`DocumentSnapshot::data`] this returns
    /// `Result<T, Error>` directly rather than wrapping it in an
    /// [`Option`]. The document's fields are deserialized with [`serde`]
    /// as a map into `T`.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] when the document's fields cannot be
    /// deserialized as `T` — for example, because `T`'s shape does not
    /// match the document's.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    /// use std::collections::HashMap;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let query_snapshot = firestore.collection("rooms")?.get().await?;
    /// for query_document_snapshot in query_snapshot {
    ///     let _data: HashMap<String, String> = query_document_snapshot.data()?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn data<T: serde::de::DeserializeOwned>(&self) -> Result<T, Error> {
        self.0.data().expect("document exists")
    }

    /// Returns whether the document exists.
    ///
    /// On a [`QueryDocumentSnapshot`] this is always `true`: query results
    /// only contain existing documents, so this type only ever wraps a
    /// [`DocumentSnapshot`] whose [`exists`](DocumentSnapshot::exists) is
    /// `true`. The method is provided for parity with
    /// [`DocumentSnapshot::exists`].
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
    ///     assert!(query_document_snapshot.exists());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn exists(&self) -> bool {
        self.0.exists()
    }

    /// Returns the ID of this document (the last segment of its path).
    ///
    /// This is the same as the ID of the [`DocumentReference`] returned by
    /// [`r#ref`](Self::r#ref); for a nested document such as
    /// `rooms/roomA/messages/msg1` it returns `"msg1"`, not the full path.
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
    pub fn id(&self) -> String {
        self.0.id()
    }

    pub fn read_time(&self) -> Timestamp {
        self.0.read_time()
    }

    pub fn r#ref(&self) -> DocumentReference {
        self.0.r#ref()
    }

    pub fn update_time(&self) -> Timestamp {
        self.0.update_time().expect("document exists")
    }
}

#[cfg(test)]
mod tests {
    use crate::DocumentReference;
    use crate::DocumentSnapshot;
    use crate::Firestore;
    use crate::FirestoreOptions;
    use crate::Timestamp;
    use firestore_path::DocumentPath;
    use std::str::FromStr as _;

    #[tokio::test]
    async fn test_new() -> anyhow::Result<()> {
        let document_path = DocumentPath::from_str("rooms/roomA")?;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let document_reference = DocumentReference::new(document_path, firestore);
        let document = serde_firestore_value::google::firestore::v1::Document {
            name: "projects/test/databases/(default)/documents/rooms/roomA".to_string(),
            fields: Default::default(),
            create_time: None,
            update_time: None,
        };
        let snapshot = DocumentSnapshot::new(Some(document), document_reference, Timestamp::now());
        assert!(snapshot.exists());
        let qds = super::QueryDocumentSnapshot::new(snapshot);
        assert!(qds.exists());
        Ok(())
    }
}
