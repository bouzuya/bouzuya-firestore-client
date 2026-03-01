use crate::Error;
use crate::collection_id::CollectionId;

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
