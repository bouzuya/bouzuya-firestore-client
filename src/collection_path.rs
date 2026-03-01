use crate::CollectionId;
use crate::DocumentId;
use crate::DocumentPath;
use crate::Error;

#[derive(Debug, thiserror::Error)]
#[error("collection path error: {0}")]
struct E(firestore_path::Error);

impl From<E> for Error {
    fn from(e: E) -> Self {
        Self::from_source(Box::new(e))
    }
}

pub struct CollectionPath(firestore_path::CollectionPath);

impl From<CollectionId> for CollectionPath {
    fn from(collection_id: CollectionId) -> Self {
        <Self as std::str::FromStr>::from_str(&collection_id.to_string())
            .expect("collection id should be valid collection path")
    }
}

impl CollectionPath {
    pub(crate) fn doc(&self, document_id: DocumentId) -> DocumentPath {
        use std::str::FromStr as _;
        DocumentPath::from_str(&format!("{}/{}", self, document_id))
            .expect("collection path and document id should form a valid document path")
    }

    pub(crate) fn id(&self) -> CollectionId {
        CollectionId::from_collection_id(self.0.collection_id().clone())
    }
}

impl std::fmt::Display for CollectionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for CollectionPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        firestore_path::CollectionPath::from_str(s)
            .map(Self)
            .map_err(E)
            .map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_doc() -> anyhow::Result<()> {
        use crate::collection_path::CollectionPath;
        use crate::document_id::DocumentId;
        use std::str::FromStr as _;
        let collection_path = CollectionPath::from_str("rooms")?;
        let document_id = DocumentId::from_str("roomA")?;
        let document_path = collection_path.doc(document_id);
        assert_eq!(document_path.to_string(), "rooms/roomA");
        Ok(())
    }

    #[test]
    fn test_id() -> anyhow::Result<()> {
        use crate::collection_path::CollectionPath;
        use std::str::FromStr as _;
        let collection_path = CollectionPath::from_str("rooms")?;
        assert_eq!(collection_path.id().to_string(), "rooms");
        let collection_path = CollectionPath::from_str("rooms/roomA/messages")?;
        assert_eq!(collection_path.id().to_string(), "messages");
        Ok(())
    }
}
