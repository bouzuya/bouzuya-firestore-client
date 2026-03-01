use crate::Error;
use crate::document_id::DocumentId;

#[derive(Debug, thiserror::Error)]
#[error("document path error: {0}")]
struct E(firestore_path::Error);

impl From<E> for Error {
    fn from(e: E) -> Self {
        Self::from_source(Box::new(e))
    }
}

pub struct DocumentPath(firestore_path::DocumentPath);

impl DocumentPath {
    pub(crate) fn id(&self) -> DocumentId {
        DocumentId::from_document_id(self.0.document_id().clone())
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
    fn test_id() -> anyhow::Result<()> {
        use crate::document_path::DocumentPath;
        use std::str::FromStr as _;
        let document_path = DocumentPath::from_str("rooms/roomA")?;
        assert_eq!(document_path.id().to_string(), "roomA");
        let document_path = DocumentPath::from_str("rooms/roomA/messages/message1")?;
        assert_eq!(document_path.id().to_string(), "message1");
        Ok(())
    }
}
