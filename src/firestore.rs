use std::str::FromStr;
use std::sync::atomic::AtomicU64;

use crate::CollectionGroup;
use crate::CollectionReference;
use crate::DocumentReference;
use crate::DocumentSnapshot;
use crate::Error;
use crate::FirestoreClient;
use crate::FirestoreOptions;
use crate::Timestamp;
use crate::Transaction;
use crate::TransactionOptions;

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

/// A client for a single Cloud Firestore database.
///
/// This is the entry point of the crate. Create one with [`Firestore::new`],
/// then obtain [`CollectionReference`]s, [`DocumentReference`]s, or
/// [`CollectionGroup`]s from it to read and write data. Use
/// [`Firestore::run_transaction`] to perform reads and writes atomically.
///
/// `Firestore` is cheap to [`Clone`]; the underlying connection is shared
/// between clones. Two clones of the same instance compare equal via
/// [`PartialEq`].
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
/// let snapshot = firestore.doc("users/alice")?.get().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct Firestore {
    firestore_client: FirestoreClient,
    id: u64,
}

impl Firestore {
    /// Creates a new [`Firestore`] client.
    ///
    /// See [`FirestoreOptions`] for how each option is resolved.
    ///
    /// # Environment variables
    ///
    /// - `GCLOUD_PROJECT`, `GOOGLE_CLOUD_PROJECT`: used as a fallback for
    ///   `project_id` when [`FirestoreOptions::project_id`] is `None`.
    /// - `FIRESTORE_EMULATOR_HOST`: if set, the client connects to the Firestore
    ///   emulator at that host instead of Google Cloud.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let firestore = Firestore::new(FirestoreOptions {
    ///     database_id: Some("my-database".to_owned()),
    ///     project_id: Some("my-project".to_owned()),
    /// })?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        FirestoreOptions {
            project_id,
            database_id,
        }: FirestoreOptions,
    ) -> Result<Self, Error> {
        let project_id = project_id
            .or_else(|| std::env::var("GCLOUD_PROJECT").ok())
            .or_else(|| std::env::var("GOOGLE_CLOUD_PROJECT").ok())
            .ok_or_else(|| Error::from_source("project_id is required".into()))?;
        let database_id = database_id.unwrap_or_else(|| "(default)".to_owned());
        let emulator_host = match std::env::var("FIRESTORE_EMULATOR_HOST") {
            Ok(firestore_emulator_host) => Some(firestore_emulator_host),
            Err(e) => match e {
                std::env::VarError::NotPresent => None,
                std::env::VarError::NotUnicode(_) => {
                    return Err(Error::from_source("FIRESTORE_EMULATOR_HOST environment variable is not a valid unicode string".into()));
                }
            },
        };
        let firestore_client = FirestoreClient::new(project_id, database_id, emulator_host)?;
        let id = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(Self {
            firestore_client,
            id,
        })
    }
}

impl Firestore {
    /// Returns a [`CollectionReference`] for the collection at the given path.
    ///
    /// `collection_path` is a slash-separated path that must point to a
    /// collection (i.e. it must have an odd number of segments), such as
    /// `"users"` or `"users/alice/posts"`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let collection_reference1 = firestore.collection("users")?;
    /// let collection_reference2 = firestore.collection("users/alice/posts")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn collection(
        &self,
        collection_path: impl Into<String>,
    ) -> Result<CollectionReference, Error> {
        let s: String = collection_path.into();
        let collection_path =
            firestore_path::CollectionPath::from_str(&s).map_err(Error::invalid_collection_path)?;
        Ok(CollectionReference::new(collection_path, self.clone()))
    }

    /// Returns a [`CollectionGroup`] that includes all collections with the
    /// given ID, regardless of their parent document.
    ///
    /// `collection_id` is a single collection ID without any slashes, such as
    /// `"posts"`. It matches every collection named `posts` anywhere in the
    /// database (e.g. `users/alice/posts`, `users/bob/posts`, ...).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let collection_group = firestore.collection_group("posts")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn collection_group(
        &self,
        collection_id: impl Into<String>,
    ) -> Result<CollectionGroup, Error> {
        let collection_id: String = collection_id.into();
        let collection_id = firestore_path::CollectionId::from_str(&collection_id)
            .map_err(Error::invalid_collection_id)?;
        Ok(CollectionGroup::new(collection_id, self.clone()))
    }

    /// Returns the database ID this client is bound to.
    ///
    /// This is the value resolved from [`FirestoreOptions::database_id`] when
    /// the client was created; if it was `None`, `"(default)"` is returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions {
    ///     database_id: Some("my-database".to_owned()),
    ///     project_id: Some("my-project".to_owned()),
    /// })?;
    /// assert_eq!(firestore.database_id(), "my-database");
    /// # Ok(())
    /// # }
    /// ```
    pub fn database_id(&self) -> String {
        self.firestore_client.database_id()
    }

    /// Returns a [`DocumentReference`] for the document at the given path.
    ///
    /// `document_path` is a slash-separated path that must point to a document
    /// (i.e. it must have an even number of segments), such as `"users/alice"`
    /// or `"users/alice/posts/post-1"`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let document_reference1 = firestore.doc("users/alice")?;
    /// let document_reference2 = firestore.doc("users/alice/posts/post-1")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn doc(&self, document_path: impl Into<String>) -> Result<DocumentReference, Error> {
        let s: String = document_path.into();
        let document_path =
            firestore_path::DocumentPath::from_str(&s).map_err(Error::invalid_document_path)?;
        Ok(DocumentReference::new(document_path, self.clone()))
    }

    /// Fetches multiple documents in a single batch request.
    ///
    /// Returns one [`DocumentSnapshot`] per input [`DocumentReference`], in the
    /// same order as the input. A snapshot is returned even if the referenced
    /// document does not exist; check [`DocumentSnapshot::exists`] to tell the
    /// two cases apart.
    ///
    /// Unlike the Node.js Firestore client's [`getAll`], this method does not
    /// accept a `ReadOptions` argument (e.g. for specifying a field mask). All
    /// fields of each document are always returned.
    ///
    /// [`getAll`]: https://docs.cloud.google.com/nodejs/docs/reference/firestore/latest/firestore/firestore#_google_cloud_firestore_Firestore_getAll_member_1_
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
    /// let snapshots = firestore
    ///     .get_all([
    ///         firestore.doc("users/alice")?,
    ///         firestore.doc("users/bob")?,
    ///     ])
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_all(
        &self,
        document_references: impl IntoIterator<Item = DocumentReference>,
    ) -> Result<Vec<DocumentSnapshot>, Error> {
        let document_references: Vec<DocumentReference> = document_references.into_iter().collect();
        let document_paths: Vec<firestore_path::DocumentPath> = document_references
            .iter()
            .map(|r| r.document_path().clone())
            .collect();
        let documents = self
            .firestore_client
            .batch_get_documents(&document_paths, None)
            .await?;
        Ok(documents
            .into_iter()
            .zip(document_references)
            .map(|((document, read_time), document_reference)| {
                DocumentSnapshot::new(
                    document,
                    document_reference,
                    Timestamp::from_prost_timestamp(read_time),
                )
            })
            .collect())
    }

    /// Lists the root-level collections of the database.
    ///
    /// Returns a [`CollectionReference`] for every top-level collection. To
    /// list subcollections of a document, use
    /// [`DocumentReference::list_collections`] instead.
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
    /// let collection_references = firestore.list_collections().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_collections(&self) -> Result<Vec<CollectionReference>, Error> {
        use std::str::FromStr as _;
        let collection_ids = self.firestore_client.list_root_collection_ids().await?;
        collection_ids
            .into_iter()
            .map(|id| {
                let collection_path = firestore_path::CollectionPath::from_str(&id)
                    .map_err(Error::invalid_collection_path)?;
                Ok(CollectionReference::new(collection_path, self.clone()))
            })
            .collect()
    }

    /// Runs `update_function` inside a Firestore transaction.
    ///
    /// A transaction is started before `update_function` is called. The
    /// function receives a [`Transaction`] on which reads and writes can be
    /// staged. When `update_function` returns:
    ///
    /// - `Ok(value)`: the staged writes are committed and `value` is returned.
    /// - `Err(e)`: the transaction is rolled back and `e` is returned.
    ///
    /// See [`TransactionOptions`] for available options (e.g. read-only vs.
    /// read-write).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    /// use bouzuya_firestore_client::TransactionOptions;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let document_reference = firestore.doc("users/alice")?;
    /// let exists = firestore
    ///     .run_transaction(
    ///         |transaction| {
    ///             let document_reference = document_reference.clone();
    ///             Box::pin(async move {
    ///                 let snapshot = transaction.get(&document_reference).await?;
    ///                 Ok::<_, bouzuya_firestore_client::Error>(snapshot.exists())
    ///             })
    ///         },
    ///         TransactionOptions::default(),
    ///     )
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run_transaction<'a, T, F>(
        &'a self,
        update_function: F,
        transaction_options: TransactionOptions,
    ) -> Result<T, Error>
    where
        F: for<'c> FnOnce(
                &'c mut Transaction,
            ) -> std::pin::Pin<
                Box<dyn std::future::Future<Output = Result<T, Error>> + Send + 'c>,
            >
            + 'a
            + Send
            + Sync,
    {
        let transaction = self
            .firestore_client
            .begin_transaction(&transaction_options)
            .await?;
        let result = async {
            let mut transaction = Transaction {
                transaction: transaction.clone(),
                writes: vec![],
            };
            let return_value = update_function(&mut transaction).await?;
            let Transaction {
                transaction,
                writes,
            } = transaction;
            self.firestore_client.commit(transaction, writes).await?;
            Ok(return_value)
        }
        .await;
        match result {
            Ok(return_value) => Ok(return_value),
            Err(e) => {
                self.firestore_client.rollback(transaction).await?;
                Err(e)
            }
        }
    }

    pub(crate) fn firestore_client(&self) -> FirestoreClient {
        self.firestore_client.clone()
    }
}

impl std::fmt::Debug for Firestore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Firestore").field("id", &self.id).finish()
    }
}

impl std::cmp::PartialEq for Firestore {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::cmp::Eq for Firestore {}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_firestore_new_database_id() -> anyhow::Result<()> {
        use crate::Firestore;
        use crate::FirestoreOptions;
        let firestore = Firestore::new(FirestoreOptions {
            database_id: Some("my-database".to_owned()),
            project_id: Some("demo-project".to_owned()),
        })?;
        assert_eq!(
            firestore
                .firestore_client()
                .database_name()
                .database_id()
                .to_string(),
            "my-database"
        );
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_firestore_client() -> anyhow::Result<()> {
        use crate::Firestore;
        use crate::FirestoreOptions;
        use firestore_path::DocumentPath;
        use std::str::FromStr as _;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let firestore_client = firestore.firestore_client();
        firestore_client
            .batch_get_documents(
                &[DocumentPath::from_str("test-collection/test-document")?],
                None,
            )
            .await?;
        Ok(())
    }
}
