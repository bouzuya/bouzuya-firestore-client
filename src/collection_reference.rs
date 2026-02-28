pub struct CollectionReference {
    collection_id: String,
}

impl CollectionReference {
    pub(crate) fn new(collection_id: String) -> Self {
        Self { collection_id }
    }
}

impl CollectionReference {
    pub fn id(&self) -> &str {
        &self.collection_id
    }
}
