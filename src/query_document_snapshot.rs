use crate::DocumentReference;
use crate::DocumentSnapshot;
use crate::Error;
use crate::Timestamp;

#[derive(Clone)]
pub struct QueryDocumentSnapshot(DocumentSnapshot);

impl QueryDocumentSnapshot {
    #[allow(dead_code)]
    pub(crate) fn new(document_snapshot: DocumentSnapshot) -> Self {
        assert!(document_snapshot.exists());
        Self(document_snapshot)
    }
}

impl QueryDocumentSnapshot {
    pub fn create_time(&self) -> Timestamp {
        self.0.create_time().expect("document exists")
    }

    pub fn data<T: serde::de::DeserializeOwned>(&self) -> Result<T, Error> {
        self.0.data().expect("document exists")
    }

    pub fn exists(&self) -> bool {
        self.0.exists()
    }

    pub fn id(&self) -> String {
        self.0.id()
    }

    pub fn r#ref(&self) -> DocumentReference {
        self.0.r#ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::DocumentReference;
    use crate::DocumentSnapshot;
    use crate::Firestore;
    use crate::FirestoreOptions;
    use firestore_path::DocumentPath;
    use std::str::FromStr as _;

    #[tokio::test]
    async fn test_new() -> anyhow::Result<()> {
        let document_path = DocumentPath::from_str("rooms/roomA")?;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let document_ref = DocumentReference::new(document_path, firestore);
        let doc = serde_firestore_value::google::firestore::v1::Document {
            name: "projects/test/databases/(default)/documents/rooms/roomA".to_string(),
            fields: Default::default(),
            create_time: None,
            update_time: None,
        };
        let snapshot = DocumentSnapshot::new(Some(doc), document_ref);
        assert!(snapshot.exists());
        let qds = super::QueryDocumentSnapshot::new(snapshot);
        assert!(qds.exists());
        Ok(())
    }
}
