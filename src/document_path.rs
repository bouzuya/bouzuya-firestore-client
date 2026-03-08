use crate::CollectionId;
use crate::CollectionPath;
use crate::Error;

#[derive(Debug, thiserror::Error)]
#[error("document path error: {0}")]
struct E(firestore_path::Error);

impl From<E> for Error {
    fn from(e: E) -> Self {
        Self::from_source(Box::new(e))
    }
}

#[derive(Clone)]
pub(crate) struct DocumentPath(firestore_path::DocumentPath);

impl DocumentPath {
    pub(crate) fn collection(&self, collection_id: CollectionId) -> CollectionPath {
        use std::str::FromStr as _;
        CollectionPath::from_str(&format!("{}/{}", self, collection_id))
            .expect("document path and collection id should form a valid collection path")
    }

    pub(crate) fn id(&self) -> firestore_path::DocumentId {
        self.0.document_id().clone()
    }

    pub(crate) fn parent(&self) -> CollectionPath {
        CollectionPath::from_collection_path(self.0.parent().clone())
    }
}

impl From<DocumentPath> for firestore_path::DocumentPath {
    fn from(document_path: DocumentPath) -> Self {
        document_path.0
    }
}

impl From<firestore_path::DocumentPath> for DocumentPath {
    fn from(document_path: firestore_path::DocumentPath) -> Self {
        Self(document_path)
    }
}

impl std::fmt::Display for DocumentPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for DocumentPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        firestore_path::DocumentPath::from_str(s)
            .map(Self)
            .map_err(E)
            .map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_clone() -> anyhow::Result<()> {
        use crate::document_path::DocumentPath;
        use std::str::FromStr as _;
        let document_path = DocumentPath::from_str("rooms/roomA")?;
        let cloned = document_path.clone();
        assert_eq!(cloned.to_string(), "rooms/roomA");
        Ok(())
    }

    #[test]
    fn test_collection() -> anyhow::Result<()> {
        use crate::collection_id::CollectionId;
        use crate::document_path::DocumentPath;
        use std::str::FromStr as _;
        let document_path = DocumentPath::from_str("rooms/roomA")?;
        let collection_id = CollectionId::from_str("messages")?;
        let collection_path = document_path.collection(collection_id);
        assert_eq!(collection_path.to_string(), "rooms/roomA/messages");
        Ok(())
    }

    #[test]
    fn test_display() -> anyhow::Result<()> {
        use crate::document_path::DocumentPath;
        use std::str::FromStr as _;
        let document_path = DocumentPath::from_str("rooms/roomA")?;
        assert_eq!(document_path.to_string(), "rooms/roomA");
        Ok(())
    }

    #[test]
    fn test_from_firestore_path_document_path() -> anyhow::Result<()> {
        use crate::document_path::DocumentPath;
        use std::str::FromStr as _;
        let inner = firestore_path::DocumentPath::from_str("rooms/roomA")?;
        let document_path = DocumentPath::from(inner);
        assert_eq!(document_path.to_string(), "rooms/roomA");
        Ok(())
    }

    #[test]
    fn test_from_self_for_firestore_path_document_path() -> anyhow::Result<()> {
        use crate::document_path::DocumentPath;
        use std::str::FromStr as _;
        let document_path = DocumentPath::from_str("rooms/roomA")?;
        let inner = firestore_path::DocumentPath::from(document_path);
        assert_eq!(inner.to_string(), "rooms/roomA");
        Ok(())
    }

    #[test]
    fn test_from_str() -> anyhow::Result<()> {
        use crate::document_path::DocumentPath;
        use std::str::FromStr as _;
        assert!(DocumentPath::from_str("rooms/roomA").is_ok());
        Ok(())
    }

    #[test]
    fn test_from_str_error() {
        use crate::document_path::DocumentPath;
        use std::str::FromStr as _;
        assert!(DocumentPath::from_str("").is_err());
    }

    #[test]
    fn test_id() -> anyhow::Result<()> {
        use crate::document_path::DocumentPath;
        use std::str::FromStr as _;
        let document_path = DocumentPath::from_str("rooms/roomA")?;
        assert_eq!(document_path.id().to_string(), "roomA");
        let document_path = DocumentPath::from_str("rooms/roomA/messages/message1")?;
        assert_eq!(document_path.id().to_string(), "message1");
        Ok(())
    }

    #[test]
    fn test_parent() -> anyhow::Result<()> {
        use crate::document_path::DocumentPath;
        use std::str::FromStr as _;
        let document_path = DocumentPath::from_str("rooms/roomA")?;
        let parent = document_path.parent();
        assert_eq!(parent.to_string(), "rooms");
        let document_path = DocumentPath::from_str("rooms/roomA/messages/message1")?;
        let parent = document_path.parent();
        assert_eq!(parent.to_string(), "rooms/roomA/messages");
        Ok(())
    }
}
