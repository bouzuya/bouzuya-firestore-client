use crate::Error;
use crate::Firestore;
use crate::IntoFieldPath;
use crate::IntoFilter;
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

    /// Returns a [`Query`] that ends at the given cursor (inclusive).
    ///
    /// `values` is matched positionally against the query's [`order_by`]
    /// clauses, so the call is typically chained with [`Query::order_by`]. The
    /// resulting query includes the document whose order-by fields equal
    /// `values`.
    ///
    /// This is a convenience for [`Query::collection_group`] followed by
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
    ///     .collection_group("posts")?
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
        Query::collection_group(self.clone()).end_at(values)
    }

    /// Returns a [`Query`] that ends before the given cursor (exclusive).
    ///
    /// `values` is matched positionally against the query's [`order_by`]
    /// clauses, so the call is typically chained with [`Query::order_by`]. The
    /// resulting query excludes the document whose order-by fields equal
    /// `values`.
    ///
    /// This is a convenience for [`Query::collection_group`] followed by
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
    ///     .collection_group("posts")?
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
        Query::collection_group(self.clone()).end_before(values)
    }

    /// Returns the [`Firestore`] this collection group belongs to.
    ///
    /// This is the same instance from which the [`CollectionGroup`] was
    /// obtained via [`Firestore::collection_group`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let collection_group = firestore.collection_group("rooms")?;
    /// assert_eq!(collection_group.firestore(), &firestore);
    /// # Ok(())
    /// # }
    /// ```
    pub fn firestore(&self) -> &Firestore {
        &self.firestore
    }

    /// Executes the collection-group query and returns its [`QuerySnapshot`].
    ///
    /// Every document in any collection whose ID matches this group is
    /// returned, regardless of its parent document. Without further filtering
    /// (e.g. [`Query::r#where`] or [`Query::limit`]) this can return a large
    /// number of documents.
    ///
    /// This is a convenience for [`Query::collection_group`] followed by
    /// [`Query::get`].
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
    /// let query_snapshot = firestore.collection_group("messages")?.get().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self) -> Result<QuerySnapshot, Error> {
        Query::collection_group(self.clone()).get().await
    }

    /// Returns a [`Query`] that returns at most `limit` documents.
    ///
    /// This is a convenience for [`Query::collection_group`] followed by
    /// [`Query::limit`]; it returns an error if `limit` is negative.
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
    ///     .collection_group("messages")?
    ///     .limit(2)?
    ///     .get()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Query::start_at
    pub fn start_at<I>(&self, values: I) -> Result<Query, Error>
    where
        I: IntoIterator,
        I::Item: serde::Serialize,
    {
        Query::collection_group(self.clone()).start_at(values)
    }

    /// Query::r#where
    pub fn r#where(&self, filter: impl IntoFilter) -> Result<Query, Error> {
        Query::collection_group(self.clone()).r#where(filter)
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
