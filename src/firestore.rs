use crate::Error;
use crate::firestore_options::FirestoreOptions;

#[derive(Clone)]
pub struct Firestore {
    _private: (),
}

impl Firestore {
    pub fn new(_options: FirestoreOptions) -> Result<Self, Error> {
        Ok(Self { _private: () })
    }
}
