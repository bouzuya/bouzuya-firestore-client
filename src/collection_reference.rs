use crate::CollectionId;
use crate::CollectionPath;
use crate::DocumentId;
use crate::DocumentReference;
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
    pub fn doc(&self, document_id: impl Into<DocumentId>) -> DocumentReference {
        DocumentReference::new(
            self.collection_path.doc(document_id.into()),
            self.firestore.clone(),
        )
    }

    pub fn id(&self) -> CollectionId {
        self.collection_path.id()
    }

    pub fn parent(&self) -> Option<DocumentReference> {
        self.collection_path.parent().map(|parent_document_path| {
            DocumentReference::new(parent_document_path, self.firestore.clone())
        })
    }

    pub fn path(&self) -> CollectionPath {
        self.collection_path.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new() -> anyhow::Result<()> {
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
