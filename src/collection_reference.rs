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
    /// Creates a new document in this collection with an auto-generated ID.
    ///
    /// A 20-character alphanumeric document ID is generated, `data` is
    /// serialized with [`serde`], and the document is created via
    /// [`DocumentReference::create`]. The returned [`DocumentReference`]
    /// points at the newly created document.
    ///
    /// Because the ID is generated client-side, calling `add` on the same
    /// collection from concurrent tasks creates distinct documents.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    /// use std::collections::HashMap;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let document_reference = firestore
    ///     .collection("rooms")?
    ///     .add(HashMap::<String, String>::new())
    ///     .await?;
    /// assert!(document_reference.path().starts_with("rooms/"));
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add(&self, data: impl serde::ser::Serialize) -> Result<DocumentReference, Error> {
        use std::str::FromStr as _;
        let s = rand::distr::SampleString::sample_string(
            &rand::distr::Alphanumeric,
            &mut rand::rand_core::UnwrapErr(rand::rngs::SysRng),
            20,
        );
        let document_id = firestore_path::DocumentId::from_str(&s)
            .expect("generated document id should be valid");
        let document_path = self
            .collection_path
            .doc(document_id)
            .map_err(Error::invalid_document_path)?;
        let document_reference = DocumentReference::new(document_path, self.firestore.clone());
        let _write_result = document_reference.create(&data).await?;
        Ok(document_reference)
    }

    /// Returns a [`DocumentReference`] to the document with the given ID in
    /// this collection.
    ///
    /// `document_id` is parsed as a Firestore document ID; invalid IDs (e.g.
    /// containing `/`, or empty) return an error. The document is not fetched
    /// or required to exist — this only constructs a reference.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let document_reference = firestore.collection("rooms")?.doc("roomA")?;
    /// assert_eq!(document_reference.id().to_string(), "roomA");
    /// # Ok(())
    /// # }
    /// ```
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

    /// Returns a [`Query`] that ends at the given cursor (inclusive).
    ///
    /// `values` is matched positionally against the query's [`order_by`]
    /// clauses, so the call is typically chained with [`Query::order_by`]. The
    /// resulting query includes the document whose order-by fields equal
    /// `values`.
    ///
    /// This is a convenience for [`Query::collection`] followed by
    /// [`Query::end_at`]; it returns an error if `values` is empty.
    ///
    /// [`order_by`]: Query::order_by
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let query_snapshot = firestore
    ///     .collection("posts")?
    ///     .end_at(vec![2_i64])?
    ///     .order_by("n", "asc")?
    ///     .get()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn end_at<I>(&self, values: I) -> Result<Query, Error>
    where
        I: IntoIterator,
        I::Item: serde::Serialize,
    {
        Query::collection(self.clone()).end_at(values)
    }

    /// Returns a [`Query`] that ends before the given cursor (exclusive).
    ///
    /// `values` is matched positionally against the query's [`order_by`]
    /// clauses, so the call is typically chained with [`Query::order_by`]. The
    /// resulting query excludes the document whose order-by fields equal
    /// `values`.
    ///
    /// This is a convenience for [`Query::collection`] followed by
    /// [`Query::end_before`]; it returns an error if `values` is empty.
    ///
    /// [`order_by`]: Query::order_by
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let query_snapshot = firestore
    ///     .collection("posts")?
    ///     .end_before(vec![2_i64])?
    ///     .order_by("n", "asc")?
    ///     .get()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn end_before<I>(&self, values: I) -> Result<Query, Error>
    where
        I: IntoIterator,
        I::Item: serde::Serialize,
    {
        Query::collection(self.clone()).end_before(values)
    }

    /// Query::firestore
    pub fn firestore(&self) -> &Firestore {
        &self.firestore
    }

    /// Query::get
    pub async fn get(&self) -> Result<QuerySnapshot, Error> {
        Query::collection(self.clone()).get().await
    }

    pub fn id(&self) -> String {
        self.collection_path.collection_id().to_string()
    }

    /// Query::limit
    pub fn limit(&self, n: i32) -> Result<Query, Error> {
        Query::collection(self.clone()).limit(n)
    }

    /// Query::offset
    pub fn offset(&self, n: i32) -> Result<Query, Error> {
        Query::collection(self.clone()).offset(n)
    }

    /// Query::order_by
    pub fn order_by(
        &self,
        field_path: impl IntoFieldPath,
        direction: &str,
    ) -> Result<Query, Error> {
        Query::collection(self.clone()).order_by(field_path, direction)
    }

    /// Query::select
    pub fn select<I>(&self, fields: I) -> Result<Query, Error>
    where
        I: IntoIterator,
        I::Item: IntoFieldPath,
    {
        Query::collection(self.clone()).select(fields)
    }

    /// Query::start_after
    pub fn start_after<I>(&self, values: I) -> Result<Query, Error>
    where
        I: IntoIterator,
        I::Item: serde::Serialize,
    {
        Query::collection(self.clone()).start_after(values)
    }

    /// Query::start_at
    pub fn start_at<I>(&self, values: I) -> Result<Query, Error>
    where
        I: IntoIterator,
        I::Item: serde::Serialize,
    {
        Query::collection(self.clone()).start_at(values)
    }

    /// Query::r#where
    pub fn r#where(&self, filter: impl IntoFilter) -> Result<Query, Error> {
        Query::collection(self.clone()).r#where(filter)
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
