use crate::Firestore;

pub struct CollectionGroup {
    collection_id: firestore_path::CollectionId,
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

impl CollectionGroup {
    pub(crate) fn collection_id(&self) -> &firestore_path::CollectionId {
        &self.collection_id
    }

    /// Query::firestore
    pub fn firestore(&self) -> &Firestore {
        &self.firestore
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_collection_id() -> anyhow::Result<()> {
        use crate::CollectionGroup;
        use crate::Firestore;
        use crate::FirestoreOptions;
        use std::str::FromStr as _;
        let collection_id = firestore_path::CollectionId::from_str("rooms")?;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let collection_group = CollectionGroup::new(collection_id, firestore);
        assert_eq!(collection_group.collection_id().to_string(), "rooms");
        Ok(())
    }

    #[tokio::test]
    async fn test_new() -> anyhow::Result<()> {
        use crate::CollectionGroup;
        use crate::Firestore;
        use crate::FirestoreOptions;
        use std::str::FromStr as _;
        let collection_id = firestore_path::CollectionId::from_str("rooms")?;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let collection_group = CollectionGroup::new(collection_id, firestore);
        assert_eq!(collection_group.collection_id().to_string(), "rooms");
        Ok(())
    }
}
