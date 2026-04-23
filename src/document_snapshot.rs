use crate::DocumentReference;
use crate::Error;
use crate::Timestamp;

#[derive(Debug, thiserror::Error)]
enum E {
    #[error("deserialize error: {0}")]
    Deserialize(#[source] serde_firestore_value::Error),
}

impl From<E> for Error {
    fn from(e: E) -> Self {
        Self::from_source(Box::new(e))
    }
}

#[derive(Clone)]
pub struct DocumentSnapshot {
    document: Option<serde_firestore_value::google::firestore::v1::Document>,
    document_reference: DocumentReference,
}

impl DocumentSnapshot {
    pub(crate) fn new(
        document: Option<serde_firestore_value::google::firestore::v1::Document>,
        document_reference: DocumentReference,
    ) -> Self {
        Self {
            document,
            document_reference,
        }
    }

    pub fn create_time(&self) -> Option<Timestamp> {
        self.document
            .as_ref()
            .and_then(|document| document.create_time)
            .map(Timestamp::from_prost_timestamp)
    }

    pub fn data<T: serde::de::DeserializeOwned>(&self) -> Option<Result<T, Error>> {
        self.document.as_ref().map(|document| {
            serde_firestore_value::from_value::<T>(
                &serde_firestore_value::google::firestore::v1::Value {
                    value_type: Some(
                        serde_firestore_value::google::firestore::v1::value::ValueType::MapValue(
                            serde_firestore_value::google::firestore::v1::MapValue {
                                fields: document.fields.clone(),
                            },
                        ),
                    ),
                },
            )
            .map_err(E::Deserialize)
            .map_err(Error::from)
        })
    }

    pub fn exists(&self) -> bool {
        self.document.is_some()
    }

    // pub fn get(&self, field_path: FieldPath) -> Option<Value> {
    //     todo!()
    // }

    pub fn id(&self) -> String {
        self.document_reference.id()
    }

    // pub fn read_time(&self) -> DateTime {
    //     todo!()
    // }

    pub fn r#ref(&self) -> DocumentReference {
        self.document_reference.clone()
    }

    pub fn update_time(&self) -> Option<Timestamp> {
        self.document
            .as_ref()
            .and_then(|document| document.update_time)
            .map(Timestamp::from_prost_timestamp)
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_new() -> anyhow::Result<()> {
        use crate::DocumentReference;
        use crate::DocumentSnapshot;
        use crate::Firestore;
        use crate::FirestoreOptions;
        use firestore_path::DocumentPath;
        use std::str::FromStr as _;
        let document_path = DocumentPath::from_str("rooms/roomA")?;
        let firestore = Firestore::new(FirestoreOptions::default())?;
        let document_reference = DocumentReference::new(document_path, firestore);
        let snapshot = DocumentSnapshot::new(None, document_reference);
        assert!(!snapshot.exists());
        assert_eq!(snapshot.id().to_string(), "roomA");
        Ok(())
    }
}
