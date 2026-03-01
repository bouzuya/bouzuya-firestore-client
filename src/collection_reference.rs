use crate::CollectionId;
use crate::CollectionPath;

pub struct CollectionReference {
    collection_path: CollectionPath,
}

impl CollectionReference {
    pub(crate) fn new(collection_path: CollectionPath) -> Self {
        Self { collection_path }
    }
}

impl CollectionReference {
    pub fn id(&self) -> CollectionId {
        self.collection_path.id()
    }
}
