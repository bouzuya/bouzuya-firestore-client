use crate::CollectionPath;
use crate::DocumentId;
use crate::DocumentReference;
use crate::Error;
use crate::Firestore;

pub struct CollectionReference {
    collection_path: CollectionPath,
    firestore: Firestore,
}

impl CollectionReference {
    pub(crate) fn new(collection_path: CollectionPath, firestore: Firestore) -> Self {
        Self {
            collection_path,
            firestore,
        }
    }
}

impl CollectionReference {
    pub fn doc(&self, document_id: impl Into<String>) -> Result<DocumentReference, Error> {
        use std::str::FromStr as _;
        let s: String = document_id.into();
        let document_id = DocumentId::from_str(&s)?;
        Ok(DocumentReference::new(
            self.collection_path.doc(document_id),
            self.firestore.clone(),
        ))
    }

    pub fn id(&self) -> String {
        self.collection_path.id().to_string()
    }

    pub fn parent(&self) -> Option<DocumentReference> {
        self.collection_path.parent().map(|parent_document_path| {
            DocumentReference::new(parent_document_path, self.firestore.clone())
        })
    }

    pub fn path(&self) -> String {
        self.collection_path.to_string()
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_new() -> anyhow::Result<()> {
        use crate::CollectionPath;
        use crate::CollectionReference;
        use crate::Firestore;
        use crate::FirestoreOptions;
        use std::str::FromStr as _;
        let collection_path = CollectionPath::from_str("rooms")?;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let collection_ref = CollectionReference::new(collection_path, firestore);
        assert_eq!(collection_ref.id().to_string(), "rooms");
        Ok(())
    }
}
