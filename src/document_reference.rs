use crate::CollectionReference;
use crate::DocumentSnapshot;
use crate::Error;
use crate::Firestore;
use crate::Precondition;
use crate::Timestamp;
use crate::WriteResult;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentReference {
    document_path: firestore_path::DocumentPath,
    firestore: Firestore,
}

impl DocumentReference {
    pub(crate) fn new(document_path: firestore_path::DocumentPath, firestore: Firestore) -> Self {
        Self {
            document_path,
            firestore,
        }
    }
}

impl DocumentReference {
    /// Returns a [`CollectionReference`] to the subcollection with the given
    /// ID under this document.
    ///
    /// `collection_id` is parsed as a Firestore collection ID; invalid IDs
    /// (e.g. containing `/`, or empty) return an error. The subcollection is
    /// not fetched or required to exist — this only constructs a reference
    /// whose path is this document's path followed by `collection_id`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let collection_reference = firestore.doc("rooms/roomA")?.collection("messages")?;
    /// assert_eq!(collection_reference.path(), "rooms/roomA/messages");
    /// # Ok(())
    /// # }
    /// ```
    pub fn collection(
        &self,
        collection_id: impl Into<String>,
    ) -> Result<CollectionReference, Error> {
        use std::str::FromStr as _;
        let s: String = collection_id.into();
        let collection_id =
            firestore_path::CollectionId::from_str(&s).map_err(Error::invalid_collection_id)?;
        Ok(CollectionReference::new(
            self.document_path
                .collection(firestore_path::CollectionPath::new(None, collection_id))
                .map_err(Error::invalid_collection_path)?,
            self.firestore.clone(),
        ))
    }

    /// Creates this document with the given `data`.
    ///
    /// `data` is serialized with [`serde`] and stored at this document's path.
    /// The write fails with an error if a document already exists at the path;
    /// use [`set`](Self::set) to write unconditionally or
    /// [`update`](Self::update) to modify an existing document.
    ///
    /// The returned [`WriteResult`] carries the server-side commit time.
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
    /// let document_reference = firestore.doc("rooms/roomA")?;
    /// let _write_result = document_reference
    ///     .create(HashMap::<String, String>::new())
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, data: impl serde::ser::Serialize) -> Result<WriteResult, Error> {
        let value =
            serde_firestore_value::to_value(&data).map_err(|e| Error::from_source(Box::new(e)))?;
        let write_time = self
            .firestore
            .firestore_client()
            .create_document(&self.document_path, value)
            .await?;
        Ok(WriteResult::new(Timestamp::from_prost_timestamp(
            write_time,
        )))
    }

    /// Deletes this document, subject to the given [`Precondition`].
    ///
    /// `precondition` lets the caller require that the document exist
    /// (`exists`) or that its last update time match (`last_update_time`);
    /// pass an empty precondition (both fields `None`) to delete
    /// unconditionally. Deleting a nonexistent document with an empty
    /// precondition succeeds without error.
    ///
    /// Subcollections of the deleted document are not deleted automatically
    /// and continue to exist independently.
    ///
    /// The returned [`WriteResult`] carries the server-side commit time.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    /// use bouzuya_firestore_client::Precondition;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let document_reference = firestore.doc("rooms/roomA")?;
    /// let _write_result = document_reference
    ///     .delete(Precondition {
    ///         exists: None,
    ///         last_update_time: None,
    ///     })
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, precondition: Precondition) -> Result<WriteResult, Error> {
        let write_time = self
            .firestore
            .firestore_client()
            .delete_document(&self.document_path, precondition)
            .await?;
        Ok(WriteResult::new(Timestamp::from_prost_timestamp(
            write_time,
        )))
    }

    pub(crate) fn document_name(&self) -> String {
        self.firestore
            .firestore_client()
            .document_name(&self.document_path)
    }

    pub(crate) fn document_path(&self) -> &firestore_path::DocumentPath {
        &self.document_path
    }

    /// Returns the [`Firestore`] this document belongs to.
    ///
    /// This is the same instance from which the [`DocumentReference`] was
    /// obtained via [`Firestore::doc`] (or by navigating from another
    /// reference within the same client).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let document_reference = firestore.doc("rooms/roomA")?;
    /// assert_eq!(document_reference.firestore(), &firestore);
    /// # Ok(())
    /// # }
    /// ```
    pub fn firestore(&self) -> &Firestore {
        &self.firestore
    }

    pub async fn get(&self) -> Result<DocumentSnapshot, Error> {
        let (document, read_time) = self
            .firestore
            .firestore_client()
            .batch_get_documents(std::slice::from_ref(&self.document_path), None)
            .await?
            .into_iter()
            .next()
            .expect("batch get documents response is missing document");
        Ok(DocumentSnapshot::new(
            document,
            self.clone(),
            Timestamp::from_prost_timestamp(read_time),
        ))
    }

    pub async fn list_collections(&self) -> Result<Vec<CollectionReference>, Error> {
        let collection_ids = self
            .firestore
            .firestore_client()
            .list_collection_ids(&self.document_path)
            .await?;
        collection_ids
            .into_iter()
            .map(|id| self.collection(id))
            .collect()
    }

    pub fn id(&self) -> String {
        self.document_path.document_id().to_string()
    }

    pub fn parent(&self) -> CollectionReference {
        CollectionReference::new(self.document_path.parent().clone(), self.firestore.clone())
    }

    pub fn path(&self) -> String {
        self.document_path.to_string()
    }

    pub async fn set(&self, data: impl serde::ser::Serialize) -> Result<WriteResult, Error> {
        let value =
            serde_firestore_value::to_value(&data).map_err(|e| Error::from_source(Box::new(e)))?;
        let write_time = self
            .firestore
            .firestore_client()
            .set_document(&self.document_path, value)
            .await?;
        Ok(WriteResult::new(Timestamp::from_prost_timestamp(
            write_time,
        )))
    }

    pub async fn update(
        &self,
        data: impl serde::ser::Serialize,
        precondition: Precondition,
    ) -> Result<WriteResult, Error> {
        let value =
            serde_firestore_value::to_value(&data).map_err(|e| Error::from_source(Box::new(e)))?;
        let write_time = self
            .firestore
            .firestore_client()
            .update_document(&self.document_path, value, precondition)
            .await?;
        Ok(WriteResult::new(Timestamp::from_prost_timestamp(
            write_time,
        )))
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_document_name() -> anyhow::Result<()> {
        use crate::DocumentReference;
        use crate::Firestore;
        use crate::FirestoreOptions;
        use firestore_path::DocumentPath;
        use std::str::FromStr as _;
        let project_id = std::env::var("GOOGLE_CLOUD_PROJECT")?;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let document_path = DocumentPath::from_str("rooms/roomA")?;
        let document_reference = DocumentReference::new(document_path, firestore);
        assert_eq!(
            document_reference.document_name(),
            format!("projects/{project_id}/databases/(default)/documents/rooms/roomA")
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_document_path() -> anyhow::Result<()> {
        use crate::DocumentReference;
        use crate::Firestore;
        use crate::FirestoreOptions;
        use firestore_path::DocumentPath;
        use std::str::FromStr as _;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let document_path = DocumentPath::from_str("rooms/roomA")?;
        let document_reference = DocumentReference::new(document_path.clone(), firestore);
        assert_eq!(document_reference.document_path(), &document_path);
        Ok(())
    }

    #[tokio::test]
    async fn test_new() -> anyhow::Result<()> {
        use crate::DocumentReference;
        use crate::Firestore;
        use crate::FirestoreOptions;
        use firestore_path::DocumentPath;
        use std::str::FromStr as _;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let document_path = DocumentPath::from_str("rooms/roomA")?;
        let document_reference = DocumentReference::new(document_path, firestore);
        assert_eq!(document_reference.id().to_string(), "roomA");
        Ok(())
    }
}
