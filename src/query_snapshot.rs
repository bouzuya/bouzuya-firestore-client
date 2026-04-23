use crate::Query;
use crate::QueryDocumentSnapshot;

#[derive(Clone)]
pub struct QuerySnapshot {
    docs: Vec<QueryDocumentSnapshot>,
    query: Query,
}

impl QuerySnapshot {
    pub(crate) fn new(query: Query, docs: Vec<QueryDocumentSnapshot>) -> Self {
        Self { docs, query }
    }
}

impl QuerySnapshot {
    pub fn docs(&self) -> Vec<QueryDocumentSnapshot> {
        self.docs.clone()
    }

    pub fn empty(&self) -> bool {
        self.docs.is_empty()
    }

    pub fn query(&self) -> Query {
        self.query.clone()
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
        let collection_ref = CollectionReference::new(collection_path, firestore);
        let query = Query::new(collection_ref);
        let _qs = QuerySnapshot::new(query, vec![]);
        Ok(())
    }
}
