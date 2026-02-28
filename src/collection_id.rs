use crate::Error;

#[derive(Debug, thiserror::Error)]
#[error("collection id error: {0}")]
struct E(firestore_path::Error);

impl From<E> for Error {
    fn from(e: E) -> Self {
        Self::from_source(Box::new(e))
    }
}

pub struct CollectionId(firestore_path::CollectionId);

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
