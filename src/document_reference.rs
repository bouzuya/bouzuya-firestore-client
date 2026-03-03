use crate::CollectionId;
use crate::CollectionReference;
use crate::DocumentId;
use crate::DocumentPath;
use crate::DocumentSnapshot;
use crate::Error;
use crate::Firestore;

#[derive(Clone)]
pub struct DocumentReference {
    document_path: DocumentPath,
    firestore: Firestore,
}

impl DocumentReference {
    pub(crate) fn new(document_path: DocumentPath, firestore: Firestore) -> Self {
        Self {
            document_path,
            firestore,
        }
    }
}

impl DocumentReference {
    pub fn collection(&self, collection_id: impl Into<CollectionId>) -> CollectionReference {
        CollectionReference::new(
            self.document_path.collection(collection_id.into()),
            self.firestore.clone(),
        )
    }

    pub async fn get(&self) -> Result<DocumentSnapshot, Error> {
        // FIXME: call GetDocument RPC API
        Ok(DocumentSnapshot::new(None, self.clone()))
    }

    pub fn id(&self) -> DocumentId {
        self.document_path.id()
    }

    pub fn parent(&self) -> CollectionReference {
        CollectionReference::new(self.document_path.parent(), self.firestore.clone())
    }

    pub fn path(&self) -> DocumentPath {
        self.document_path.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new() -> anyhow::Result<()> {
        use crate::DocumentPath;
        use crate::DocumentReference;
        use crate::Firestore;
        use crate::FirestoreOptions;
        use std::str::FromStr as _;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let document_path = DocumentPath::from_str("rooms/roomA")?;
        let document_ref = DocumentReference::new(document_path, firestore);
        assert_eq!(document_ref.id().to_string(), "roomA");
        Ok(())
    }
}
