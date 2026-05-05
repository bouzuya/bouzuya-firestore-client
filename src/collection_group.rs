use crate::Firestore;

pub struct CollectionGroup {
    #[allow(dead_code)]
    collection_id: firestore_path::CollectionId,
    #[allow(dead_code)]
    firestore: Firestore,
}

impl CollectionGroup {
    pub(crate) fn new(collection_id: firestore_path::CollectionId, firestore: Firestore) -> Self {
        Self {
            collection_id,
            firestore,
        }
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_new() -> anyhow::Result<()> {
        use crate::CollectionGroup;
        use crate::Firestore;
        use crate::FirestoreOptions;
        use std::str::FromStr as _;
        let collection_id = firestore_path::CollectionId::from_str("rooms")?;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let _collection_group = CollectionGroup::new(collection_id, firestore);
        // NOTE: can't test CollectionGroup::collection_id() because it's private
        Ok(())
    }
}
