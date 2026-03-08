use crate::Error;

#[derive(Debug, thiserror::Error)]
#[error("collection path error: {0}")]
struct E(firestore_path::Error);

impl From<E> for Error {
    fn from(e: E) -> Self {
        Self::from_source(Box::new(e))
    }
}

#[derive(Clone)]
pub(crate) struct CollectionPath(firestore_path::CollectionPath);

impl CollectionPath {
    pub(crate) fn doc(
        &self,
        document_id: firestore_path::DocumentId,
    ) -> firestore_path::DocumentPath {
        use std::str::FromStr as _;
        firestore_path::DocumentPath::from_str(&format!("{}/{}", self, document_id))
            .expect("collection path and document id should form a valid document path")
    }

    pub(crate) fn id(&self) -> firestore_path::CollectionId {
        self.0.collection_id().clone()
    }

    pub(crate) fn parent(&self) -> Option<firestore_path::DocumentPath> {
        self.0.parent().map(|parent| {
            use std::str::FromStr as _;
            firestore_path::DocumentPath::from_str(&parent.to_string())
                .expect("collection path's parent should be a valid document path")
        })
    }
}

impl From<firestore_path::CollectionId> for CollectionPath {
    fn from(collection_id: firestore_path::CollectionId) -> Self {
        <Self as std::str::FromStr>::from_str(&collection_id.to_string())
            .expect("collection id should be valid collection path")
    }
}

impl From<CollectionPath> for firestore_path::CollectionPath {
    fn from(collection_path: CollectionPath) -> Self {
        collection_path.0
    }
}

impl From<firestore_path::CollectionPath> for CollectionPath {
    fn from(collection_path: firestore_path::CollectionPath) -> Self {
        Self(collection_path)
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
    fn test_clone() -> anyhow::Result<()> {
        use crate::collection_path::CollectionPath;
        use std::str::FromStr as _;
        let collection_path = CollectionPath::from_str("rooms/roomA/messages")?;
        let cloned = collection_path.clone();
        assert_eq!(cloned.to_string(), "rooms/roomA/messages");
        Ok(())
    }

    #[test]
    fn test_display() -> anyhow::Result<()> {
        use crate::collection_path::CollectionPath;
        use std::str::FromStr as _;
        let collection_path = CollectionPath::from_str("rooms/roomA/messages")?;
        assert_eq!(collection_path.to_string(), "rooms/roomA/messages");
        Ok(())
    }

    #[test]
    fn test_doc() -> anyhow::Result<()> {
        use crate::CollectionPath;
        use firestore_path::DocumentId;
        use std::str::FromStr as _;
        let collection_path = CollectionPath::from_str("rooms")?;
        let document_id = DocumentId::from_str("roomA")?;
        let document_path = collection_path.doc(document_id);
        assert_eq!(document_path.to_string(), "rooms/roomA");
        Ok(())
    }

    #[test]
    fn test_from_collection_id() -> anyhow::Result<()> {
        use crate::collection_path::CollectionPath;
        use firestore_path::CollectionId;
        use std::str::FromStr as _;
        let collection_id = CollectionId::from_str("rooms")?;
        let collection_path = CollectionPath::from(collection_id);
        assert_eq!(collection_path.to_string(), "rooms");
        Ok(())
    }

    #[test]
    fn test_from_firestore_path_collection_path() -> anyhow::Result<()> {
        use crate::collection_path::CollectionPath;
        use std::str::FromStr as _;
        let inner = firestore_path::CollectionPath::from_str("rooms")?;
        let collection_path = CollectionPath::from(inner);
        assert_eq!(collection_path.to_string(), "rooms");
        Ok(())
    }

    #[test]
    fn test_from_self_for_firestore_path_collection_path() -> anyhow::Result<()> {
        use crate::collection_path::CollectionPath;
        use std::str::FromStr as _;
        let collection_path = CollectionPath::from_str("rooms")?;
        let inner = firestore_path::CollectionPath::from(collection_path);
        assert_eq!(inner.to_string(), "rooms");
        Ok(())
    }

    #[test]
    fn test_from_str() -> anyhow::Result<()> {
        use crate::collection_path::CollectionPath;
        use std::str::FromStr as _;
        let collection_path = CollectionPath::from_str("rooms/roomA/messages")?;
        assert_eq!(collection_path.to_string(), "rooms/roomA/messages");
        Ok(())
    }

    #[test]
    fn test_from_str_error() {
        use crate::collection_path::CollectionPath;
        use std::str::FromStr as _;
        assert!(CollectionPath::from_str("").is_err());
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

    #[test]
    fn test_parent() -> anyhow::Result<()> {
        use crate::collection_path::CollectionPath;
        use anyhow::Context as _;
        use std::str::FromStr as _;
        let collection_path = CollectionPath::from_str("rooms/roomA/messages")?;
        let parent = collection_path
            .parent()
            .context("collection path should have a parent")?;
        assert_eq!(parent.to_string(), "rooms/roomA");
        let collection_path = CollectionPath::from_str("rooms")?;
        assert!(collection_path.parent().is_none());
        Ok(())
    }
}
