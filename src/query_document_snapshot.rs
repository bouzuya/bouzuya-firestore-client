#[allow(clippy::manual_non_exhaustive)]
pub struct QueryDocumentSnapshot {
    _private: (),
}

impl QueryDocumentSnapshot {
    pub fn exists(&self) -> bool {
        true
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
