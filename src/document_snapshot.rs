use crate::DocumentReference;
use crate::Error;
use crate::Timestamp;

#[derive(Debug, thiserror::Error)]
enum E {
    #[error("deserialize error: {0}")]
    Deserialize(#[source] serde_firestore_value::Error),
}

impl From<E> for Error {
    fn from(e: E) -> Self {
        Self::from_source(Box::new(e))
    }
}

/// A point-in-time snapshot of a single document.
///
/// A `DocumentSnapshot` captures the state of one document at the moment it
/// was read from the server (see [`read_time`](Self::read_time)). Obtain one
/// with [`DocumentReference::get`], [`Transaction::get`], or
/// [`Firestore::get_all`]; a [`QuerySnapshot`] is made up of
/// [`QueryDocumentSnapshot`]s, each of which carries a `DocumentSnapshot`'s
/// view of its document.
///
/// Reading a nonexistent document is not an error — it produces a snapshot
/// whose [`exists`](Self::exists) is `false`. Such a snapshot has no
/// [`data`](Self::data), no [`create_time`](Self::create_time), and no
/// [`update_time`](Self::update_time); only [`id`](Self::id),
/// [`ref`](Self::ref), and [`read_time`](Self::read_time) are meaningful.
/// When the document does exist, [`data`](Self::data) deserializes its
/// fields into a user type.
///
/// [`Clone`] performs a deep copy of the document's fields, so it is *not*
/// free for documents with substantial payloads (a Firestore document can
/// be up to ~1 MiB). Prefer borrowing the snapshot, or extracting only the
/// pieces you need (e.g. via [`data`](Self::data)), when working with
/// large documents.
///
/// [`Firestore::get_all`]: crate::Firestore::get_all
/// [`QueryDocumentSnapshot`]: crate::QueryDocumentSnapshot
/// [`QuerySnapshot`]: crate::QuerySnapshot
/// [`Transaction::get`]: crate::Transaction::get
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
/// let snapshot = firestore.doc("rooms/roomA")?.get().await?;
/// if let Some(data) = snapshot.data::<HashMap<String, String>>() {
///     let _data = data?;
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct DocumentSnapshot {
    document: Option<serde_firestore_value::google::firestore::v1::Document>,
    document_reference: DocumentReference,
    read_time: Timestamp,
}

impl DocumentSnapshot {
    pub(crate) fn new(
        document: Option<serde_firestore_value::google::firestore::v1::Document>,
        document_reference: DocumentReference,
        read_time: Timestamp,
    ) -> Self {
        Self {
            document,
            document_reference,
            read_time,
        }
    }

    /// Returns the server-side time at which this document was created, or
    /// [`None`] if it does not exist.
    ///
    /// A snapshot taken of a nonexistent document (see [`exists`](Self::exists))
    /// has no create time and returns [`None`]. For an existing document, the
    /// returned [`Timestamp`] is the commit time of the original
    /// [`DocumentReference::create`] or [`DocumentReference::set`] that brought
    /// the document into being, not the time of the most recent write — use
    /// [`update_time`](Self::update_time) for that.
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
    /// let snapshot = firestore.doc("rooms/roomA")?.get().await?;
    /// if let Some(create_time) = snapshot.create_time() {
    ///     let _ = create_time;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_time(&self) -> Option<Timestamp> {
        self.document
            .as_ref()
            .and_then(|document| document.create_time)
            .map(Timestamp::from_prost_timestamp)
    }

    /// Deserializes this document's fields into `T`, or returns [`None`] if
    /// the document does not exist.
    ///
    /// A snapshot taken of a nonexistent document (see [`exists`](Self::exists))
    /// has no data and returns [`None`]. For an existing document, its fields
    /// are deserialized with [`serde`] as a map into `T`; if deserialization
    /// fails (for example because `T`'s shape does not match the document's),
    /// the inner [`Result`] is [`Err`].
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
    /// let snapshot = firestore.doc("rooms/roomA")?.get().await?;
    /// if let Some(data) = snapshot.data::<HashMap<String, String>>() {
    ///     let _data = data?;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn data<T: serde::de::DeserializeOwned>(&self) -> Option<Result<T, Error>> {
        self.document.as_ref().map(|document| {
            serde_firestore_value::from_value::<T>(
                &serde_firestore_value::google::firestore::v1::Value {
                    value_type: Some(
                        serde_firestore_value::google::firestore::v1::value::ValueType::MapValue(
                            serde_firestore_value::google::firestore::v1::MapValue {
                                fields: document.fields.clone(),
                            },
                        ),
                    ),
                },
            )
            .map_err(E::Deserialize)
            .map_err(Error::from)
        })
    }

    /// Returns whether the document existed at the time this snapshot was
    /// taken.
    ///
    /// Fetching a nonexistent document with [`DocumentReference::get`] is not
    /// an error — it returns a snapshot whose `exists` is `false`. Such a
    /// snapshot has no [`data`](Self::data), no [`create_time`](Self::create_time),
    /// and no [`update_time`](Self::update_time); only [`id`](Self::id),
    /// [`ref`](Self::ref) and [`read_time`](Self::read_time) are meaningful.
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
    /// let snapshot = firestore.doc("rooms/roomA")?.get().await?;
    /// if snapshot.exists() {
    ///     // ...
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn exists(&self) -> bool {
        self.document.is_some()
    }

    // pub fn get(&self, field_path: FieldPath) -> Option<Value> {
    //     todo!()
    // }

    /// Returns the ID of this document (the last segment of its path).
    ///
    /// This is the same as the ID of the [`DocumentReference`] this snapshot
    /// was taken from (see [`ref`](Self::ref) and [`DocumentReference::id`]).
    /// The ID is available even when the document does not
    /// [`exists`](Self::exists), because it comes from the reference, not the
    /// fetched document. For a nested document such as
    /// `rooms/roomA/messages/msg1` this returns `"msg1"`, not the full path.
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
    /// let snapshot = firestore.doc("rooms/roomA")?.get().await?;
    /// assert_eq!(snapshot.id(), "roomA");
    /// # Ok(())
    /// # }
    /// ```
    pub fn id(&self) -> String {
        self.document_reference.id()
    }

    /// Returns the [`DocumentReference`] this snapshot was taken from.
    ///
    /// The returned reference is a clone of the one passed to
    /// [`DocumentReference::get`] (or the equivalent inside a transaction or
    /// query); use it to re-read the document, write it, or navigate to a
    /// subcollection. The reference is available even when the document does
    /// not [`exists`](Self::exists).
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
    /// let snapshot = firestore.doc("rooms/roomA")?.get().await?;
    /// assert_eq!(snapshot.r#ref().path(), "rooms/roomA");
    /// # Ok(())
    /// # }
    /// ```
    pub fn r#ref(&self) -> DocumentReference {
        self.document_reference.clone()
    }

    /// Returns the time at which this snapshot was read from the server.
    ///
    /// Unlike [`create_time`](Self::create_time) and
    /// [`update_time`](Self::update_time), the read time is always available —
    /// even for a snapshot of a nonexistent document (see
    /// [`exists`](Self::exists)) — because it describes the read itself, not
    /// the document. Two snapshots of the same document taken in succession
    /// will share a [`create_time`](Self::create_time) and
    /// [`update_time`](Self::update_time) (assuming no intervening write) but
    /// will have different read times.
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
    /// let snapshot = firestore.doc("rooms/roomA")?.get().await?;
    /// let _read_time = snapshot.read_time();
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_time(&self) -> Timestamp {
        self.read_time
    }

    /// Returns the server-side time at which this document was last updated,
    /// or [`None`] if it does not exist.
    ///
    /// A snapshot taken of a nonexistent document (see [`exists`](Self::exists))
    /// has no update time and returns [`None`]. For an existing document, the
    /// returned [`Timestamp`] is the commit time of the most recent write that
    /// changed it — [`DocumentReference::set`], [`DocumentReference::update`],
    /// or the [`DocumentReference::create`] that brought it into being if no
    /// later write has happened. For a freshly created document with no
    /// subsequent writes, `update_time` equals
    /// [`create_time`](Self::create_time).
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
    /// let snapshot = firestore.doc("rooms/roomA")?.get().await?;
    /// if let Some(update_time) = snapshot.update_time() {
    ///     let _ = update_time;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn update_time(&self) -> Option<Timestamp> {
        self.document
            .as_ref()
            .and_then(|document| document.update_time)
            .map(Timestamp::from_prost_timestamp)
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_new() -> anyhow::Result<()> {
        use crate::DocumentReference;
        use crate::DocumentSnapshot;
        use crate::Firestore;
        use crate::FirestoreOptions;
        use crate::Timestamp;
        use firestore_path::DocumentPath;
        use std::str::FromStr as _;
        let document_path = DocumentPath::from_str("rooms/roomA")?;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let document_reference = DocumentReference::new(document_path, firestore);
        let read_time = Timestamp::now();
        let snapshot = DocumentSnapshot::new(None, document_reference, read_time);
        assert!(!snapshot.exists());
        assert_eq!(snapshot.id().to_string(), "roomA");
        Ok(())
    }
}
