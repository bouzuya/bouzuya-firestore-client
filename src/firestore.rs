use crate::CollectionReference;
use crate::Error;
use crate::FirestoreOptions;

#[derive(Clone)]
pub struct Firestore {
    _private: (),
}

impl Firestore {
    pub fn new(_options: FirestoreOptions) -> Result<Self, Error> {
        Ok(Self { _private: () })
    }
}

impl Firestore {
    // FIXME: collection_id should be validated
    // FIXME: collection_path support
    pub fn collection(&self, collection_id: &str) -> CollectionReference {
        CollectionReference::new(collection_id.to_string())
    }
}
