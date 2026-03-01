use crate::CollectionId;
use crate::CollectionReference;
use crate::DocumentId;
use crate::DocumentPath;

pub struct DocumentReference {
    document_path: DocumentPath,
}

impl DocumentReference {
    pub(crate) fn new(document_path: DocumentPath) -> Self {
        Self { document_path }
    }
}

impl DocumentReference {
    pub fn collection(&self, collection_id: impl Into<CollectionId>) -> CollectionReference {
        CollectionReference::new(self.document_path.collection(collection_id.into()))
    }

    pub fn id(&self) -> DocumentId {
        self.document_path.id()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new() -> anyhow::Result<()> {
        use crate::document_path::DocumentPath;
        use crate::document_reference::DocumentReference;
        use std::str::FromStr as _;
        let document_path = DocumentPath::from_str("rooms/roomA")?;
        let document_ref = DocumentReference::new(document_path);
        assert_eq!(document_ref.id().to_string(), "roomA");
        Ok(())
    }
}
