use crate::CollectionPath;
use crate::CollectionReference;
use crate::DocumentPath;
use crate::DocumentReference;
use crate::Error;
use crate::FirestoreClient;
use crate::FirestoreOptions;

#[derive(Clone)]
pub struct Firestore {
    firestore_client: FirestoreClient,
}

impl Firestore {
    pub fn new(_options: FirestoreOptions) -> Result<Self, Error> {
        // FIXME: Use options
        let firestore_client = FirestoreClient::new("(default)".to_owned())?;
        Ok(Self { firestore_client })
    }
}

impl Firestore {
    pub fn collection(&self, collection_path: impl Into<CollectionPath>) -> CollectionReference {
        CollectionReference::new(collection_path.into(), self.clone())
    }

    pub fn doc(&self, document_path: impl Into<DocumentPath>) -> DocumentReference {
        DocumentReference::new(document_path.into(), self.clone())
    }

    pub(crate) fn firestore_client(&self) -> FirestoreClient {
        self.firestore_client.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_firestore_client() -> anyhow::Result<()> {
        use crate::Firestore;
        use crate::FirestoreOptions;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let _firestore_client = firestore.firestore_client();
        Ok(())
    }
}
