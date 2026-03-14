use crate::DocumentReference;
use crate::DocumentSnapshot;
use crate::Error;
use crate::Precondition;
use crate::google;

pub struct Transaction {
    pub(crate) transaction: Vec<u8>,
    pub(crate) writes: Vec<google::firestore::v1::Write>,
}

impl Transaction {
    pub fn create(
        &mut self,
        document_ref: &DocumentReference,
        data: &impl serde::ser::Serialize,
    ) -> Result<(), Error> {
        let value =
            serde_firestore_value::to_value(data).map_err(|e| Error::from_source(Box::new(e)))?;
        let fields = match value.value_type {
            Some(google::firestore::v1::value::ValueType::MapValue(map_value)) => map_value.fields,
            _ => return Err(Error::from_source("value must be a map".into())),
        };
        self.writes.push(google::firestore::v1::Write {
            update_mask: None,
            update_transforms: vec![],
            current_document: Some(google::firestore::v1::Precondition {
                condition_type: Some(google::firestore::v1::precondition::ConditionType::Exists(
                    false,
                )),
            }),
            operation: Some(google::firestore::v1::write::Operation::Update(
                google::firestore::v1::Document {
                    name: document_ref.document_name(),
                    fields,
                    create_time: None,
                    update_time: None,
                },
            )),
        });
        Ok(())
    }

    pub fn delete(
        &mut self,
        document_ref: &DocumentReference,
        Precondition {
            exists,
            last_update_time,
        }: Precondition,
    ) -> Result<(), Error> {
        let current_document = match (exists, last_update_time) {
            (None, None) => None,
            (None, Some(last_update_time)) => Some(google::firestore::v1::Precondition {
                condition_type: Some(
                    google::firestore::v1::precondition::ConditionType::UpdateTime(
                        last_update_time.into_prost_timestamp(),
                    ),
                ),
            }),
            (Some(exists), None) => Some(google::firestore::v1::Precondition {
                condition_type: Some(google::firestore::v1::precondition::ConditionType::Exists(
                    exists,
                )),
            }),
            (Some(_), Some(_)) => {
                return Err(Error::from_source(
                    "precondition cannot have both exists and last_update_time".into(),
                ));
            }
        };
        self.writes.push(google::firestore::v1::Write {
            update_mask: None,
            update_transforms: vec![],
            current_document,
            operation: Some(google::firestore::v1::write::Operation::Delete(
                document_ref.document_name(),
            )),
        });
        Ok(())
    }

    // TODO: Query support
    pub async fn get(&self, document_ref: &DocumentReference) -> Result<DocumentSnapshot, Error> {
        let document = document_ref
            .firestore()
            .firestore_client()
            .get_document_in_transaction(document_ref.document_path(), self.transaction.clone())
            .await?;
        Ok(DocumentSnapshot::new(document, document_ref.clone()))
    }

    pub fn set(
        &mut self,
        document_ref: &DocumentReference,
        data: &impl serde::ser::Serialize,
    ) -> Result<(), Error> {
        let value =
            serde_firestore_value::to_value(data).map_err(|e| Error::from_source(Box::new(e)))?;
        let fields = match value.value_type {
            Some(google::firestore::v1::value::ValueType::MapValue(map_value)) => map_value.fields,
            _ => return Err(Error::from_source("value must be a map".into())),
        };
        self.writes.push(google::firestore::v1::Write {
            update_mask: None,
            update_transforms: vec![],
            current_document: None,
            operation: Some(google::firestore::v1::write::Operation::Update(
                google::firestore::v1::Document {
                    name: document_ref.document_name(),
                    fields,
                    create_time: None,
                    update_time: None,
                },
            )),
        });
        Ok(())
    }

    pub fn update(
        &mut self,
        document_ref: &DocumentReference,
        data: &impl serde::ser::Serialize,
        Precondition {
            exists,
            last_update_time,
        }: Precondition,
    ) -> Result<(), Error> {
        let value =
            serde_firestore_value::to_value(data).map_err(|e| Error::from_source(Box::new(e)))?;
        let fields = match value.value_type {
            Some(google::firestore::v1::value::ValueType::MapValue(map_value)) => map_value.fields,
            _ => return Err(Error::from_source("value must be a map".into())),
        };
        let current_document = match (exists, last_update_time) {
            // default to exists: true if no precondition is provided, since update requires the document to exist
            (None, None) => Some(google::firestore::v1::Precondition {
                condition_type: Some(google::firestore::v1::precondition::ConditionType::Exists(
                    true,
                )),
            }),
            (None, Some(last_update_time)) => Some(google::firestore::v1::Precondition {
                condition_type: Some(
                    google::firestore::v1::precondition::ConditionType::UpdateTime(
                        last_update_time.into_prost_timestamp(),
                    ),
                ),
            }),
            (Some(exists), None) => Some(google::firestore::v1::Precondition {
                condition_type: Some(google::firestore::v1::precondition::ConditionType::Exists(
                    exists,
                )),
            }),
            (Some(_), Some(_)) => {
                return Err(Error::from_source(
                    "precondition cannot have both exists and last_update_time".into(),
                ));
            }
        };
        let field_paths = fields.keys().cloned().collect();
        self.writes.push(google::firestore::v1::Write {
            update_mask: Some(google::firestore::v1::DocumentMask { field_paths }),
            update_transforms: vec![],
            current_document,
            operation: Some(google::firestore::v1::write::Operation::Update(
                google::firestore::v1::Document {
                    name: document_ref.document_name(),
                    fields,
                    create_time: None,
                    update_time: None,
                },
            )),
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction() {
        let transaction = Transaction {
            transaction: vec![1, 2, 3],
            writes: vec![],
        };
        assert_eq!(transaction.transaction, vec![1, 2, 3]);
        assert_eq!(transaction.writes.len(), 0);
    }

    #[test]
    fn test_writes() {
        let transaction = Transaction {
            transaction: vec![],
            writes: vec![google::firestore::v1::Write::default()],
        };
        assert_eq!(transaction.writes.len(), 1);
    }
}
