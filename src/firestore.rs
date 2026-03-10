use std::str::FromStr;

use crate::CollectionReference;
use crate::DocumentReference;
use crate::Error;
use crate::FirestoreClient;
use crate::FirestoreOptions;
use crate::Transaction;
use crate::TransactionOptions;

#[derive(Clone)]
pub struct Firestore {
    firestore_client: FirestoreClient,
}

impl Firestore {
    pub fn new(_options: FirestoreOptions) -> Result<Self, Error> {
        let emulator_host = match std::env::var("FIRESTORE_EMULATOR_HOST") {
            Ok(firestore_emulator_host) => Some(firestore_emulator_host),
            Err(e) => match e {
                std::env::VarError::NotPresent => None,
                std::env::VarError::NotUnicode(_) => {
                    return Err(Error::from_source("FIRESTORE_EMULATOR_HOST environment variable is not a valid unicode string".into()));
                }
            },
        };
        // FIXME: Use options
        let firestore_client = FirestoreClient::new(
            "projects/demo-project/databases/(default)".to_owned(),
            emulator_host,
        )?;
        Ok(Self { firestore_client })
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

    pub fn doc(&self, document_path: impl Into<String>) -> Result<DocumentReference, Error> {
        let s: String = document_path.into();
        let document_path =
            firestore_path::DocumentPath::from_str(&s).map_err(Error::invalid_document_path)?;
        Ok(DocumentReference::new(document_path, self.clone()))
    }

    pub async fn run_transaction<'a, T, F>(
        &'a self,
        update_function: F,
        _transaction_options: TransactionOptions,
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
        // FIXME: Use transaction options
        let transaction = self
            .firestore_client
            .begin_transaction(&_transaction_options)
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

#[cfg(test)]
mod tests {
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
