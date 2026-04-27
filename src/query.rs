use crate::CollectionReference;
use crate::DocumentReference;
use crate::DocumentSnapshot;
use crate::Error;
use crate::Firestore;
use crate::QueryDocumentSnapshot;
use crate::QuerySnapshot;
use crate::google;

#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    collection_path: firestore_path::CollectionPath,
    firestore: Firestore,
    query: firestore_structured_query::Query,
}

impl Query {
    pub(crate) fn new(collection_reference: CollectionReference) -> Self {
        let firestore = collection_reference.firestore().clone();
        let collection_path = <firestore_path::CollectionPath as std::str::FromStr>::from_str(
            &collection_reference.path(),
        )
        .expect("collection_reference has valid path");
        let query = firestore_structured_query::Query::collection(collection_reference.id());
        Self {
            collection_path,
            firestore,
            query,
        }
    }
}

impl Query {
    pub async fn get(&self) -> Result<QuerySnapshot, Error> {
        // collection query
        let firestore_client = self.firestore.firestore_client();
        let documents = firestore_client
            .run_query(
                &self.collection_path,
                google::firestore::v1::StructuredQuery::from(self.query.clone()),
            )
            .await?;
        let query_document_snapshots = documents
            .into_iter()
            .map(|document| {
                let document_name =
                    <firestore_path::DocumentName as std::str::FromStr>::from_str(&document.name)
                        .map_err(|e| Error::from_source(Box::new(e)))?;
                let document_path = firestore_path::DocumentPath::from(document_name);
                let document_reference =
                    DocumentReference::new(document_path, self.firestore.clone());
                let document_snapshot = DocumentSnapshot::new(Some(document), document_reference);
                Ok(QueryDocumentSnapshot::new(document_snapshot))
            })
            .collect::<Result<Vec<QueryDocumentSnapshot>, Error>>()?;
        Ok(QuerySnapshot::new(self.clone(), query_document_snapshots))
    }

    pub fn limit(&self, n: i32) -> Query {
        Query {
            collection_path: self.collection_path.clone(),
            firestore: self.firestore.clone(),
            query: self.query.clone().limit(n),
        }
    }

    pub fn offset(&self, n: i32) -> Query {
        Query {
            collection_path: self.collection_path.clone(),
            firestore: self.firestore.clone(),
            query: self.query.clone().offset(n),
        }
    }

    pub fn r#where(&self, filter: crate::Filter) -> Query {
        Query {
            collection_path: self.collection_path.clone(),
            firestore: self.firestore.clone(),
            query: self.query.clone().r#where(filter.into_inner()),
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
        use crate::Query;
        use firestore_path::CollectionPath;
        use std::str::FromStr as _;
        let collection_path = CollectionPath::from_str("rooms")?;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let collection_reference = CollectionReference::new(collection_path, firestore);
        let _query = Query::new(collection_reference);
        Ok(())
    }
}
