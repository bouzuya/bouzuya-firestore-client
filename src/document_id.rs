use crate::Error;

#[derive(Debug, thiserror::Error)]
#[error("document id error: {0}")]
struct E(firestore_path::Error);

impl From<E> for Error {
    fn from(e: E) -> Self {
        Self::from_source(Box::new(e))
    }
}

pub struct DocumentId(firestore_path::DocumentId);

impl DocumentId {
    pub(crate) fn from_document_id(document_id: firestore_path::DocumentId) -> Self {
        Self(document_id)
    }
}

impl std::fmt::Display for DocumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for DocumentId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        firestore_path::DocumentId::from_str(s)
            .map(Self)
            .map_err(E)
            .map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_from_document_id() -> anyhow::Result<()> {
        use crate::document_id::DocumentId;
        use std::str::FromStr as _;
        let inner = firestore_path::DocumentId::from_str("roomA")?;
        let document_id = DocumentId::from_document_id(inner);
        assert_eq!(document_id.to_string(), "roomA");
        Ok(())
    }
}
