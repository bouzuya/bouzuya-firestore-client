use crate::CollectionPath;
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
    pub fn collection(&self, collection_path: impl Into<CollectionPath>) -> CollectionReference {
        CollectionReference::new(collection_path.into())
    }
}
