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
    order_by: Vec<firestore_structured_query::Order>,
    query: firestore_structured_query::Query,
    where_: Vec<firestore_structured_query::Filter>,
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
            order_by: Vec::new(),
            query,
            where_: Vec::new(),
        }
    }
}

impl Query {
    pub async fn get(&self) -> Result<QuerySnapshot, Error> {
        // collection query
        let firestore_client = self.firestore.firestore_client();
        let mut query = self.query.clone().order_by(self.order_by.clone());
        match self.where_.len() {
            0 => {}
            1 => {
                query = query.r#where(self.where_[0].clone());
            }
            _ => {
                query = query.r#where(firestore_structured_query::Filter::and(
                    self.where_.iter().cloned(),
                ));
            }
        }
        let documents = firestore_client
            .run_query(
                &self.collection_path,
                google::firestore::v1::StructuredQuery::from(query),
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
            order_by: self.order_by.clone(),
            query: self.query.clone().limit(n),
            where_: self.where_.clone(),
        }
    }

    pub fn offset(&self, n: i32) -> Query {
        Query {
            collection_path: self.collection_path.clone(),
            firestore: self.firestore.clone(),
            order_by: self.order_by.clone(),
            query: self.query.clone().offset(n),
            where_: self.where_.clone(),
        }
    }

    #[allow(private_bounds)]
    pub fn order_by(
        &self,
        field_path: impl crate::IntoFieldPath,
        direction: &str,
    ) -> Result<Query, Error> {
        let field_path = field_path.into_field_path()?;
        let field_path = firestore_structured_query::FieldPath::raw(field_path.to_string());
        let order = match direction {
            "asc" => field_path.ascending(),
            "desc" => field_path.descending(),
            _ => {
                return Err(Error::custom(format!(
                    "unsupported direction: {}",
                    direction
                )));
            }
        };
        let mut order_by = self.order_by.clone();
        order_by.push(order);
        Ok(Query {
            collection_path: self.collection_path.clone(),
            firestore: self.firestore.clone(),
            order_by,
            query: self.query.clone(),
            where_: self.where_.clone(),
        })
    }

    pub fn start_after<I>(&self, values: I) -> Result<Query, Error>
    where
        I: IntoIterator,
        I::Item: serde::Serialize,
    {
        let values = values
            .into_iter()
            .map(|v| {
                serde_firestore_value::to_value(&v).map_err(|e| Error::from_source(Box::new(e)))
            })
            .collect::<Result<Vec<_>, _>>()?;
        if values.is_empty() {
            return Err(Error::custom("start_after requires at least one value"));
        }
        Ok(Query {
            collection_path: self.collection_path.clone(),
            firestore: self.firestore.clone(),
            order_by: self.order_by.clone(),
            query: self.query.clone().start_after(values),
            where_: self.where_.clone(),
        })
    }

    pub fn r#where(&self, filter: crate::Filter) -> Query {
        let mut where_ = self.where_.clone();
        where_.push(filter.into_inner());
        Query {
            collection_path: self.collection_path.clone(),
            firestore: self.firestore.clone(),
            order_by: self.order_by.clone(),
            query: self.query.clone(),
            where_,
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
