use crate::CollectionReference;

#[allow(clippy::manual_non_exhaustive)]
pub struct Query {
    collection_reference: CollectionReference,
    #[allow(dead_code)]
    limit: Option<i32>,
}

impl Query {
    pub(crate) fn new(collection_reference: CollectionReference) -> Self {
        Self {
            collection_reference,
            limit: None,
        }
    }
}

impl Query {
    pub fn limit(&self, n: i32) -> Query {
        Query {
            collection_reference: self.collection_reference.clone(),
            limit: Some(n),
        }
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_new() -> anyhow::Result<()> {
        use crate::CollectionReference;
        use crate::Firestore;
        use crate::FirestoreOptions;
        use firestore_path::CollectionPath;
        use std::str::FromStr as _;
        let collection_path = CollectionPath::from_str("rooms")?;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let collection_ref = CollectionReference::new(collection_path, firestore);
        let _query = crate::Query::new(collection_ref);
        Ok(())
    }
}
