use crate::DocumentReference;
use crate::DocumentSnapshot;
use crate::Error;
use crate::Precondition;
use crate::Timestamp;
use crate::google;

/// A reference to a transaction.
///
/// The `Transaction` object passed to a transaction's update function provides
/// the methods to read and write data within the transaction context. See
/// [`Firestore::run_transaction`](crate::Firestore::run_transaction).
///
/// Reads ([`get`](Self::get)) execute immediately against the server and hold
/// a pessimistic lock on the returned documents. Writes
/// ([`create`](Self::create), [`set`](Self::set), [`update`](Self::update),
/// [`delete`](Self::delete)) are staged on the transaction and applied
/// atomically when the surrounding `run_transaction` commits; if the update
/// function returns an error, the transaction is rolled back instead. In a
/// read-write transaction, all reads must precede any writes.
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use bouzuya_firestore_client::Firestore;
/// use bouzuya_firestore_client::FirestoreOptions;
/// use bouzuya_firestore_client::TransactionOptions;
///
/// let firestore = Firestore::new(FirestoreOptions::default())?;
/// let document_reference = firestore.doc("rooms/roomA")?;
/// let exists = firestore
///     .run_transaction(
///         |transaction| {
///             let document_reference = document_reference.clone();
///             Box::pin(async move {
///                 let snapshot = transaction.get(&document_reference).await?;
///                 Ok::<_, bouzuya_firestore_client::Error>(snapshot.exists())
///             })
///         },
///         TransactionOptions::default(),
///     )
///     .await?;
/// let _ = exists;
/// # Ok(())
/// # }
/// ```
pub struct Transaction {
    pub(crate) transaction: Vec<u8>,
    pub(crate) writes: Vec<google::firestore::v1::Write>,
}

impl Transaction {
    /// Creates the document referred to by the provided `document_reference`
    /// within this transaction.
    ///
    /// `data` is serialized with [`serde`] and must serialize to a map. The
    /// write is staged on this transaction and applied when the surrounding
    /// [`Firestore::run_transaction`](crate::Firestore::run_transaction)
    /// commits; the transaction fails if a document already exists at the
    /// specified location. Use [`set`](Self::set) to write unconditionally or
    /// [`update`](Self::update) to modify an existing document.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] when `data` fails to serialize or does not
    /// serialize to a map.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    /// use bouzuya_firestore_client::TransactionOptions;
    /// use std::collections::HashMap;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let document_reference = firestore.doc("rooms/roomA")?;
    /// firestore
    ///     .run_transaction(
    ///         |transaction| {
    ///             let document_reference = document_reference.clone();
    ///             Box::pin(async move {
    ///                 transaction.create(
    ///                     &document_reference,
    ///                     &HashMap::<String, String>::new(),
    ///                 )?;
    ///                 Ok(())
    ///             })
    ///         },
    ///         TransactionOptions::default(),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(
        &mut self,
        document_reference: &DocumentReference,
        data: &impl serde::ser::Serialize,
    ) -> Result<(), Error> {
        let value =
            serde_firestore_value::to_value(data).map_err(|e| Error::from_source(Box::new(e)))?;
        let fields = match value.value_type {
            Some(google::firestore::v1::value::ValueType::MapValue(map_value)) => map_value.fields,
            _ => return Err(Error::from_source("value must be a map".into())),
        };
        self.writes.push(google::firestore::v1::Write {
            update_mask: None,
            update_transforms: vec![],
            current_document: Some(google::firestore::v1::Precondition {
                condition_type: Some(google::firestore::v1::precondition::ConditionType::Exists(
                    false,
                )),
            }),
            operation: Some(google::firestore::v1::write::Operation::Update(
                google::firestore::v1::Document {
                    name: document_reference.document_name(),
                    fields,
                    create_time: None,
                    update_time: None,
                },
            )),
        });
        Ok(())
    }

    /// Deletes the document referred to by the provided `document_reference`
    /// within this transaction, subject to the given `precondition`.
    ///
    /// `precondition` lets the caller require that the document exist
    /// (`exists`) or that its last update time match (`last_update_time`);
    /// pass [`Precondition::default`] to delete unconditionally. The delete
    /// is staged on this transaction and applied when the surrounding
    /// [`Firestore::run_transaction`](crate::Firestore::run_transaction)
    /// commits.
    ///
    /// Subcollections of the deleted document are not deleted automatically
    /// and continue to exist independently.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] when `precondition` sets both `exists` and
    /// `last_update_time`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    /// use bouzuya_firestore_client::Precondition;
    /// use bouzuya_firestore_client::TransactionOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let document_reference = firestore.doc("rooms/roomA")?;
    /// firestore
    ///     .run_transaction(
    ///         |transaction| {
    ///             let document_reference = document_reference.clone();
    ///             Box::pin(async move {
    ///                 transaction.delete(&document_reference, Precondition::default())?;
    ///                 Ok(())
    ///             })
    ///         },
    ///         TransactionOptions::default(),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete(
        &mut self,
        document_reference: &DocumentReference,
        Precondition {
            exists,
            last_update_time,
        }: Precondition,
    ) -> Result<(), Error> {
        let current_document = match (exists, last_update_time) {
            (None, None) => None,
            (None, Some(last_update_time)) => Some(google::firestore::v1::Precondition {
                condition_type: Some(
                    google::firestore::v1::precondition::ConditionType::UpdateTime(
                        last_update_time.into_prost_timestamp(),
                    ),
                ),
            }),
            (Some(exists), None) => Some(google::firestore::v1::Precondition {
                condition_type: Some(google::firestore::v1::precondition::ConditionType::Exists(
                    exists,
                )),
            }),
            (Some(_), Some(_)) => {
                return Err(Error::from_source(
                    "precondition cannot have both exists and last_update_time".into(),
                ));
            }
        };
        self.writes.push(google::firestore::v1::Write {
            update_mask: None,
            update_transforms: vec![],
            current_document,
            operation: Some(google::firestore::v1::write::Operation::Delete(
                document_reference.document_name(),
            )),
        });
        Ok(())
    }

    // TODO: Query support
    /// Reads the document referenced by the provided `document_reference`
    /// within this transaction. Holds a pessimistic lock on the returned
    /// document.
    ///
    /// In a read-write transaction, all reads must precede any writes; call
    /// [`create`](Self::create), [`set`](Self::set), [`update`](Self::update),
    /// or [`delete`](Self::delete) only after every `get` has completed.
    ///
    /// The returned [`DocumentSnapshot`] is empty if the document does not
    /// exist; check with [`DocumentSnapshot::exists`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] when the read RPC fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    /// use bouzuya_firestore_client::TransactionOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let document_reference = firestore.doc("rooms/roomA")?;
    /// let snapshot = firestore
    ///     .run_transaction(
    ///         |transaction| {
    ///             let document_reference = document_reference.clone();
    ///             Box::pin(async move { transaction.get(&document_reference).await })
    ///         },
    ///         TransactionOptions::default(),
    ///     )
    ///     .await?;
    /// let _ = snapshot.exists();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(
        &self,
        document_reference: &DocumentReference,
    ) -> Result<DocumentSnapshot, Error> {
        let (document, read_time) = document_reference
            .firestore()
            .firestore_client()
            .batch_get_documents(
                std::slice::from_ref(document_reference.document_path()),
                Some(self.transaction.clone()),
            )
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Error::from_source("batch get documents response is missing".into()))?;
        Ok(DocumentSnapshot::new(
            document,
            document_reference.clone(),
            Timestamp::from_prost_timestamp(read_time),
        ))
    }

    /// Writes to the document referred to by the provided
    /// `document_reference` within this transaction. If the document does
    /// not exist yet, it will be created.
    ///
    /// `data` is serialized with [`serde`] and must serialize to a map. The
    /// write replaces the whole document — fields that exist on the previous
    /// document but not in `data` are removed. The write is staged on this
    /// transaction and applied when the surrounding
    /// [`Firestore::run_transaction`](crate::Firestore::run_transaction)
    /// commits. Use [`create`](Self::create) to fail when the document
    /// already exists, or [`update`](Self::update) to modify only the fields
    /// you pass.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] when `data` fails to serialize or does not
    /// serialize to a map.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    /// use bouzuya_firestore_client::TransactionOptions;
    /// use std::collections::HashMap;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let document_reference = firestore.doc("rooms/roomA")?;
    /// firestore
    ///     .run_transaction(
    ///         |transaction| {
    ///             let document_reference = document_reference.clone();
    ///             Box::pin(async move {
    ///                 transaction.set(
    ///                     &document_reference,
    ///                     &HashMap::from([("a".to_string(), "1".to_string())]),
    ///                 )?;
    ///                 Ok(())
    ///             })
    ///         },
    ///         TransactionOptions::default(),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set(
        &mut self,
        document_reference: &DocumentReference,
        data: &impl serde::ser::Serialize,
    ) -> Result<(), Error> {
        let value =
            serde_firestore_value::to_value(data).map_err(|e| Error::from_source(Box::new(e)))?;
        let fields = match value.value_type {
            Some(google::firestore::v1::value::ValueType::MapValue(map_value)) => map_value.fields,
            _ => return Err(Error::from_source("value must be a map".into())),
        };
        self.writes.push(google::firestore::v1::Write {
            update_mask: None,
            update_transforms: vec![],
            current_document: None,
            operation: Some(google::firestore::v1::write::Operation::Update(
                google::firestore::v1::Document {
                    name: document_reference.document_name(),
                    fields,
                    create_time: None,
                    update_time: None,
                },
            )),
        });
        Ok(())
    }

    /// Updates fields in the document referred to by the provided
    /// `document_reference` within this transaction, subject to the given
    /// `precondition`.
    ///
    /// `precondition` lets the caller require that the document exist
    /// (`exists`) or that its last update time match (`last_update_time`);
    /// pass [`Precondition::default`] to default to "document must exist",
    /// matching the Node.js Admin SDK's `Transaction.update` behavior. The
    /// write is staged on this transaction and applied when the surrounding
    /// [`Firestore::run_transaction`](crate::Firestore::run_transaction)
    /// commits.
    ///
    /// # Behavior in v4.0 (known bug)
    ///
    /// In the current implementation, `data` is serialized with [`serde`]
    /// and the whole document is overwritten — fields that exist on the
    /// previous document but not in `data` are removed. This makes `update`
    /// equivalent to [`set`](Self::set) with an added [`Precondition`].
    ///
    /// The intended behavior is partial update (merge), matching the
    /// `Transaction.update` method in the Firebase Admin SDK for Node.js:
    /// only fields present in `data` should be written, and other fields
    /// should be left untouched. The current behavior is a bug and will be
    /// fixed in a future release; until then, callers that need merge
    /// semantics should [`get`](Self::get) the document inside the
    /// transaction and merge client-side before calling this method.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] when `data` fails to serialize, does not
    /// serialize to a map, or when `precondition` sets both `exists` and
    /// `last_update_time`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    /// use bouzuya_firestore_client::Precondition;
    /// use bouzuya_firestore_client::TransactionOptions;
    /// use std::collections::HashMap;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let document_reference = firestore.doc("rooms/roomA")?;
    /// firestore
    ///     .run_transaction(
    ///         |transaction| {
    ///             let document_reference = document_reference.clone();
    ///             Box::pin(async move {
    ///                 transaction.update(
    ///                     &document_reference,
    ///                     &HashMap::from([("a".to_string(), "updated".to_string())]),
    ///                     Precondition::default(),
    ///                 )?;
    ///                 Ok(())
    ///             })
    ///         },
    ///         TransactionOptions::default(),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update(
        &mut self,
        document_reference: &DocumentReference,
        data: &impl serde::ser::Serialize,
        Precondition {
            exists,
            last_update_time,
        }: Precondition,
    ) -> Result<(), Error> {
        let value =
            serde_firestore_value::to_value(data).map_err(|e| Error::from_source(Box::new(e)))?;
        let fields = match value.value_type {
            Some(google::firestore::v1::value::ValueType::MapValue(map_value)) => map_value.fields,
            _ => return Err(Error::from_source("value must be a map".into())),
        };
        let current_document = match (exists, last_update_time) {
            // default to exists: true if no precondition is provided, since update requires the document to exist
            (None, None) => Some(google::firestore::v1::Precondition {
                condition_type: Some(google::firestore::v1::precondition::ConditionType::Exists(
                    true,
                )),
            }),
            (None, Some(last_update_time)) => Some(google::firestore::v1::Precondition {
                condition_type: Some(
                    google::firestore::v1::precondition::ConditionType::UpdateTime(
                        last_update_time.into_prost_timestamp(),
                    ),
                ),
            }),
            (Some(exists), None) => Some(google::firestore::v1::Precondition {
                condition_type: Some(google::firestore::v1::precondition::ConditionType::Exists(
                    exists,
                )),
            }),
            (Some(_), Some(_)) => {
                return Err(Error::from_source(
                    "precondition cannot have both exists and last_update_time".into(),
                ));
            }
        };
        let field_paths = fields.keys().cloned().collect();
        self.writes.push(google::firestore::v1::Write {
            update_mask: Some(google::firestore::v1::DocumentMask { field_paths }),
            update_transforms: vec![],
            current_document,
            operation: Some(google::firestore::v1::write::Operation::Update(
                google::firestore::v1::Document {
                    name: document_reference.document_name(),
                    fields,
                    create_time: None,
                    update_time: None,
                },
            )),
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction() {
        let transaction = Transaction {
            transaction: vec![1, 2, 3],
            writes: vec![],
        };
        assert_eq!(transaction.transaction, vec![1, 2, 3]);
        assert_eq!(transaction.writes.len(), 0);
    }

    #[test]
    fn test_writes() {
        let transaction = Transaction {
            transaction: vec![],
            writes: vec![google::firestore::v1::Write::default()],
        };
        assert_eq!(transaction.writes.len(), 1);
    }
}
