use crate::Query;
use crate::QueryDocumentSnapshot;

#[derive(Clone)]
pub struct QuerySnapshot {
    query: Query,
    query_document_snapshots: Vec<QueryDocumentSnapshot>,
}

impl QuerySnapshot {
    pub(crate) fn new(query: Query, query_document_snapshots: Vec<QueryDocumentSnapshot>) -> Self {
        Self {
            query,
            query_document_snapshots,
        }
    }
}

impl QuerySnapshot {
    pub fn docs(&self) -> Vec<QueryDocumentSnapshot> {
        self.query_document_snapshots.clone()
    }

    pub fn empty(&self) -> bool {
        self.query_document_snapshots.is_empty()
    }

    pub fn query(&self) -> Query {
        self.query.clone()
    }

    pub fn size(&self) -> usize {
        self.query_document_snapshots.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::CollectionReference;
    use crate::Firestore;
    use crate::FirestoreOptions;
    use crate::Query;
    use crate::QuerySnapshot;
    use firestore_path::CollectionPath;
    use std::str::FromStr as _;

    #[tokio::test]
    async fn test_new() -> anyhow::Result<()> {
        let collection_path = CollectionPath::from_str("rooms")?;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let collection_reference = CollectionReference::new(collection_path, firestore);
        let query = Query::new(collection_reference);
        let _qs = QuerySnapshot::new(query, vec![]);
        Ok(())
    }
}
