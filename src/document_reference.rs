use crate::CollectionReference;
use crate::DocumentSnapshot;
use crate::Error;
use crate::Firestore;
use crate::Precondition;
use crate::WriteResult;

#[derive(Clone)]
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

    pub async fn create(&self, data: impl serde::ser::Serialize) -> Result<WriteResult, Error> {
        let value =
            serde_firestore_value::to_value(&data).map_err(|e| Error::from_source(Box::new(e)))?;
        let write_time = self
            .firestore
            .firestore_client()
            .create_document(&self.document_path, value)
            .await?;
        Ok(WriteResult::new(crate::Timestamp::from_prost_timestamp(
            write_time,
        )))
    }

    pub async fn delete(&self, precondition: Precondition) -> Result<WriteResult, Error> {
        let write_time = self
            .firestore
            .firestore_client()
            .delete_document(&self.document_path, precondition)
            .await?;
        Ok(WriteResult::new(crate::Timestamp::from_prost_timestamp(
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

    pub fn firestore(&self) -> &Firestore {
        &self.firestore
    }

    pub async fn get(&self) -> Result<DocumentSnapshot, Error> {
        let document = self
            .firestore
            .firestore_client()
            .get_document(&self.document_path)
            .await?;
        Ok(DocumentSnapshot::new(document, self.clone()))
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
        let document_ref = DocumentReference::new(document_path, firestore);
        assert_eq!(
            document_ref.document_name(),
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
        let document_ref = DocumentReference::new(document_path.clone(), firestore);
        assert_eq!(document_ref.document_path(), &document_path);
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
        let document_ref = DocumentReference::new(document_path, firestore);
        assert_eq!(document_ref.id().to_string(), "roomA");
        Ok(())
    }
}
