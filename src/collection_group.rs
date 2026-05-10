use crate::Error;
use crate::Firestore;
use crate::IntoFieldPath;
use crate::Query;
use crate::QuerySnapshot;

#[derive(Clone)]
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

    /// Query::end_at
    pub fn end_at<I>(&self, values: I) -> Result<Query, Error>
    where
        I: IntoIterator,
        I::Item: serde::Serialize,
    {
        Query::collection_group(self.clone()).end_at(values)
    }

    /// Query::end_before
    pub fn end_before<I>(&self, values: I) -> Result<Query, Error>
    where
        I: IntoIterator,
        I::Item: serde::Serialize,
    {
        Query::collection_group(self.clone()).end_before(values)
    }

    /// Query::firestore
    pub fn firestore(&self) -> &Firestore {
        &self.firestore
    }

    /// Query::get
    pub async fn get(&self) -> Result<QuerySnapshot, Error> {
        Query::collection_group(self.clone()).get().await
    }

    /// Query::limit
    pub fn limit(&self, limit: i32) -> Result<Query, Error> {
        Query::collection_group(self.clone()).limit(limit)
    }

    /// Query::offset
    pub fn offset(&self, offset: i32) -> Result<Query, Error> {
        Query::collection_group(self.clone()).offset(offset)
    }

    /// Query::order_by
    pub fn order_by(
        &self,
        field_path: impl IntoFieldPath,
        direction: &str,
    ) -> Result<Query, Error> {
        Query::collection_group(self.clone()).order_by(field_path, direction)
    }

    /// Query::select
    pub fn select<I>(&self, fields: I) -> Result<Query, Error>
    where
        I: IntoIterator,
        I::Item: IntoFieldPath,
    {
        Query::collection_group(self.clone()).select(fields)
    }

    /// Query::start_after
    pub fn start_after<I>(&self, values: I) -> Result<Query, Error>
    where
        I: IntoIterator,
        I::Item: serde::Serialize,
    {
        Query::collection_group(self.clone()).start_after(values)
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
