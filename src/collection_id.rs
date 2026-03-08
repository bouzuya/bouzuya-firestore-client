use crate::Error;

#[derive(Debug, thiserror::Error)]
#[error("collection id error: {0}")]
struct E(firestore_path::Error);

impl From<E> for Error {
    fn from(e: E) -> Self {
        Self::from_source(Box::new(e))
    }
}

pub(crate) struct CollectionId(firestore_path::CollectionId);

impl CollectionId {
    pub(crate) fn from_collection_id(collection_id: firestore_path::CollectionId) -> Self {
        Self(collection_id)
    }
}

impl From<CollectionId> for firestore_path::CollectionId {
    fn from(collection_id: CollectionId) -> Self {
        collection_id.0
    }
}

impl From<firestore_path::CollectionId> for CollectionId {
    fn from(collection_id: firestore_path::CollectionId) -> Self {
        Self(collection_id)
    }
}

impl std::fmt::Display for CollectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for CollectionId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        firestore_path::CollectionId::from_str(s)
            .map(Self)
            .map_err(E)
            .map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_display() -> anyhow::Result<()> {
        use crate::collection_id::CollectionId;
        use std::str::FromStr as _;
        let collection_id = CollectionId::from_str("rooms")?;
        assert_eq!(collection_id.to_string(), "rooms");
        Ok(())
    }

    #[test]
    fn test_from_collection_id() -> anyhow::Result<()> {
        use crate::collection_id::CollectionId;
        use std::str::FromStr as _;
        let inner = firestore_path::CollectionId::from_str("rooms")?;
        let collection_id = CollectionId::from_collection_id(inner);
        assert_eq!(collection_id.to_string(), "rooms");
        Ok(())
    }

    #[test]
    fn test_from_firestore_path_collection_id() -> anyhow::Result<()> {
        use crate::collection_id::CollectionId;
        use std::str::FromStr as _;
        let inner = firestore_path::CollectionId::from_str("rooms")?;
        let collection_id = CollectionId::from(inner);
        assert_eq!(collection_id.to_string(), "rooms");
        Ok(())
    }

    #[test]
    fn test_from_self_for_firestore_path_collection_id() -> anyhow::Result<()> {
        use crate::collection_id::CollectionId;
        use std::str::FromStr as _;
        let collection_id = CollectionId::from_str("rooms")?;
        let inner = firestore_path::CollectionId::from(collection_id);
        assert_eq!(inner.to_string(), "rooms");
        Ok(())
    }

    #[test]
    fn test_from_str() {
        use crate::collection_id::CollectionId;
        use std::str::FromStr as _;
        assert!(CollectionId::from_str("rooms").is_ok());
    }

    #[test]
    fn test_from_str_error() {
        use crate::collection_id::CollectionId;
        use std::str::FromStr as _;
        assert!(CollectionId::from_str("").is_err());
    }
}
