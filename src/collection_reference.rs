use crate::DocumentReference;
use crate::Error;
use crate::Firestore;
use crate::IntoFieldPath;
use crate::IntoFilter;
use crate::Query;
use crate::QuerySnapshot;

#[derive(Clone)]
pub struct CollectionReference {
    collection_path: firestore_path::CollectionPath,
    firestore: Firestore,
}

impl CollectionReference {
    pub(crate) fn new(
        collection_path: firestore_path::CollectionPath,
        firestore: Firestore,
    ) -> Self {
        Self {
            collection_path,
            firestore,
        }
    }
}

impl CollectionReference {
    pub async fn add(&self, data: impl serde::ser::Serialize) -> Result<DocumentReference, Error> {
        let s = rand::distr::SampleString::sample_string(
            &rand::distr::Alphanumeric,
            &mut rand::rand_core::UnwrapErr(rand::rngs::SysRng),
            20,
        );
        let document_id = <firestore_path::DocumentId as std::str::FromStr>::from_str(&s)
            .expect("generated document id should be valid");
        let document_path = self
            .collection_path
            .doc(document_id)
            .map_err(Error::invalid_document_path)?;
        let document_reference = DocumentReference::new(document_path, self.firestore.clone());
        let _write_result = document_reference.create(&data).await?;
        Ok(document_reference)
    }

    pub fn doc(&self, document_id: impl Into<String>) -> Result<DocumentReference, Error> {
        use std::str::FromStr as _;
        let s: String = document_id.into();
        let document_id =
            firestore_path::DocumentId::from_str(&s).map_err(Error::invalid_document_id)?;
        Ok(DocumentReference::new(
            self.collection_path
                .doc(document_id)
                .map_err(Error::invalid_document_path)?,
            self.firestore.clone(),
        ))
    }

    /// Query::firestore
    pub fn firestore(&self) -> &Firestore {
        &self.firestore
    }

    /// Query::get
    pub async fn get(&self) -> Result<QuerySnapshot, Error> {
        Query::new(self.clone()).get().await
    }

    pub fn id(&self) -> String {
        self.collection_path.collection_id().to_string()
    }

    /// Query::limit
    pub fn limit(&self, n: i32) -> Result<Query, Error> {
        Query::new(self.clone()).limit(n)
    }

    /// Query::offset
    pub fn offset(&self, n: i32) -> Result<Query, Error> {
        Query::new(self.clone()).offset(n)
    }

    /// Query::order_by
    #[allow(private_bounds)]
    pub fn order_by(
        &self,
        field_path: impl IntoFieldPath,
        direction: &str,
    ) -> Result<Query, Error> {
        Query::new(self.clone()).order_by(field_path, direction)
    }

    /// Query::select
    #[allow(private_bounds)]
    pub fn select<I>(&self, fields: I) -> Result<Query, Error>
    where
        I: IntoIterator,
        I::Item: IntoFieldPath,
    {
        Query::new(self.clone()).select(fields)
    }

    /// Query::start_after
    pub fn start_after<I>(&self, values: I) -> Result<Query, Error>
    where
        I: IntoIterator,
        I::Item: serde::Serialize,
    {
        Query::new(self.clone()).start_after(values)
    }

    /// Query::start_at
    pub fn start_at<I>(&self, values: I) -> Result<Query, Error>
    where
        I: IntoIterator,
        I::Item: serde::Serialize,
    {
        Query::new(self.clone()).start_at(values)
    }

    /// Query::r#where
    #[allow(private_bounds)]
    pub fn r#where(&self, filter: impl IntoFilter) -> Result<Query, Error> {
        Query::new(self.clone()).r#where(filter)
    }

    pub async fn list_documents(&self) -> Result<Vec<DocumentReference>, Error> {
        let document_ids = self
            .firestore
            .firestore_client()
            .list_documents(&self.collection_path)
            .await?;
        Ok(document_ids
            .into_iter()
            .map(|it| DocumentReference::new(it, self.firestore.clone()))
            .collect())
    }

    pub fn parent(&self) -> Option<DocumentReference> {
        self.collection_path.parent().map(|parent_document_path| {
            DocumentReference::new(parent_document_path.clone(), self.firestore.clone())
        })
    }

    pub fn path(&self) -> String {
        self.collection_path.to_string()
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
        let collection_reference = CollectionReference::new(collection_path, firestore);
        assert_eq!(collection_reference.id().to_string(), "rooms");
        Ok(())
    }
}
