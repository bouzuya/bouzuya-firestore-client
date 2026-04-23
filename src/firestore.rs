use std::str::FromStr;
use std::sync::atomic::AtomicU64;

use crate::CollectionReference;
use crate::DocumentReference;
use crate::DocumentSnapshot;
use crate::Error;
use crate::FirestoreClient;
use crate::FirestoreOptions;
use crate::Transaction;
use crate::TransactionOptions;

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
pub struct Firestore {
    firestore_client: FirestoreClient,
    id: u64,
}

impl Firestore {
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
    pub fn collection(
        &self,
        collection_path: impl Into<String>,
    ) -> Result<CollectionReference, Error> {
        let s: String = collection_path.into();
        let collection_path =
            firestore_path::CollectionPath::from_str(&s).map_err(Error::invalid_collection_path)?;
        Ok(CollectionReference::new(collection_path, self.clone()))
    }

    pub async fn get_all(
        &self,
        document_refs: impl IntoIterator<Item = DocumentReference>,
    ) -> Result<Vec<DocumentSnapshot>, Error> {
        let document_refs: Vec<DocumentReference> = document_refs.into_iter().collect();
        let document_paths: Vec<firestore_path::DocumentPath> = document_refs
            .iter()
            .map(|r| r.document_path().clone())
            .collect();
        let documents = self.firestore_client.batch_get(&document_paths).await?;
        Ok(documents
            .into_iter()
            .zip(document_refs)
            .map(|(doc, doc_ref)| DocumentSnapshot::new(doc, doc_ref))
            .collect())
    }

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

    pub fn doc(&self, document_path: impl Into<String>) -> Result<DocumentReference, Error> {
        let s: String = document_path.into();
        let document_path =
            firestore_path::DocumentPath::from_str(&s).map_err(Error::invalid_document_path)?;
        Ok(DocumentReference::new(document_path, self.clone()))
    }

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
            .get_document(&DocumentPath::from_str("test-collection/test-document")?)
            .await?;
        Ok(())
    }
}
