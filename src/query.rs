use crate::CollectionReference;
use crate::DocumentReference;
use crate::DocumentSnapshot;
use crate::Error;
use crate::Firestore;
use crate::QueryDocumentSnapshot;
use crate::QuerySnapshot;
use crate::google;

#[derive(Clone)]
pub struct Query {
    collection_reference: CollectionReference,
    firestore: Firestore,
    limit: Option<i32>,
    offset: Option<i32>,
}

impl Query {
    pub(crate) fn new(collection_reference: CollectionReference) -> Self {
        let firestore = collection_reference.firestore().clone();
        Self {
            collection_reference,
            firestore,
            limit: None,
            offset: None,
        }
    }
}

impl Query {
    pub async fn get(&self) -> Result<QuerySnapshot, Error> {
        let collection_path = <firestore_path::CollectionPath as std::str::FromStr>::from_str(
            &self.collection_reference.path(),
        )
        .map_err(Error::invalid_collection_path)?;
        // collection query
        let fsq = firestore_structured_query::Query::collection(self.collection_reference.id());
        let fsq = match self.limit {
            Some(n) => fsq.limit(n),
            None => fsq,
        };
        let fsq = match self.offset {
            Some(n) => fsq.offset(n),
            None => fsq,
        };
        let structured_query = google::firestore::v1::StructuredQuery::from(fsq);
        let firestore_client = self.firestore.firestore_client();
        let documents = firestore_client
            .run_query(&collection_path, structured_query)
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
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(QuerySnapshot::new(self.clone(), query_document_snapshots))
    }

    pub fn limit(&self, n: i32) -> Query {
        Query {
            collection_reference: self.collection_reference.clone(),
            firestore: self.firestore.clone(),
            limit: Some(n),
            offset: self.offset,
        }
    }

    pub fn offset(&self, n: i32) -> Query {
        Query {
            collection_reference: self.collection_reference.clone(),
            firestore: self.firestore.clone(),
            limit: self.limit,
            offset: Some(n),
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
