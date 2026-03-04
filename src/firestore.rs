use std::str::FromStr;

use crate::CollectionPath;
use crate::CollectionReference;
use crate::DocumentPath;
use crate::DocumentReference;
use crate::Error;
use crate::FirestoreClient;
use crate::FirestoreOptions;

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
    pub fn collection(&self, collection_path: impl Into<CollectionPath>) -> CollectionReference {
        CollectionReference::new(collection_path.into(), self.clone())
    }

    pub fn doc(&self, document_path: impl Into<String>) -> Result<DocumentReference, Error> {
        let s: String = document_path.into();
        let document_path =
            DocumentPath::from_str(&s).map_err(|e| Error::from_source(Box::new(e)))?;
        Ok(DocumentReference::new(document_path, self.clone()))
    }

    pub(crate) fn firestore_client(&self) -> FirestoreClient {
        self.firestore_client.clone()
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_firestore_client() -> anyhow::Result<()> {
        use crate::DocumentPath;
        use crate::Firestore;
        use crate::FirestoreOptions;
        use std::str::FromStr as _;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let firestore_client = firestore.firestore_client();
        firestore_client
            .get_document(&DocumentPath::from_str("test-collection/test-document")?)
            .await?;
        Ok(())
    }
}
