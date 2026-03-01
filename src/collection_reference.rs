use crate::CollectionId;
use crate::CollectionPath;
use crate::DocumentId;
use crate::DocumentReference;

pub struct CollectionReference {
    collection_path: CollectionPath,
}

impl CollectionReference {
    pub(crate) fn new(collection_path: CollectionPath) -> Self {
        Self { collection_path }
    }
}

impl CollectionReference {
    pub fn doc(&self, document_id: impl Into<DocumentId>) -> DocumentReference {
        DocumentReference::new(self.collection_path.doc(document_id.into()))
    }

    pub fn id(&self) -> CollectionId {
        self.collection_path.id()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new() -> anyhow::Result<()> {
        use crate::collection_path::CollectionPath;
        use crate::collection_reference::CollectionReference;
        use std::str::FromStr as _;
        let collection_path = CollectionPath::from_str("rooms")?;
        let collection_ref = CollectionReference::new(collection_path);
        assert_eq!(collection_ref.id().to_string(), "rooms");
        Ok(())
    }
}
