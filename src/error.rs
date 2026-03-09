#[derive(Debug, thiserror::Error)]
enum E {
    #[error("invalid collection id")]
    InvalidCollectionId(#[source] firestore_path::Error),
    #[error("invalid document id")]
    InvalidDocumentId(#[source] firestore_path::Error),
    #[error("invalid document path")]
    InvalidDocumentPath(#[source] firestore_path::Error),
    #[error("unknown")]
    Unknown(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, thiserror::Error)]
#[error("firestore error: {0}")]
pub struct Error(#[source] E);

impl Error {
    pub(crate) fn from_source(source: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self(E::Unknown(source))
    }

    pub(crate) fn invalid_collection_id(e: firestore_path::Error) -> Self {
        Self(E::InvalidCollectionId(e))
    }

    pub(crate) fn invalid_document_id(e: firestore_path::Error) -> Self {
        Self(E::InvalidDocumentId(e))
    }

    pub(crate) fn invalid_document_path(e: firestore_path::Error) -> Self {
        Self(E::InvalidDocumentPath(e))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_from_source() {
        use crate::error::Error;
        #[derive(Debug, thiserror::Error)]
        #[error("test error")]
        struct TestError;
        let error = Error::from_source(Box::new(TestError));
        assert_eq!(error.to_string(), "firestore error: unknown");
    }

    #[test]
    fn test_invalid_collection_id() {
        use crate::error::Error;
        let firestore_path_error =
            <firestore_path::CollectionId as std::str::FromStr>::from_str("").unwrap_err();
        let error = Error::invalid_collection_id(firestore_path_error);
        assert_eq!(error.to_string(), "firestore error: invalid collection id");
    }

    #[test]
    fn test_invalid_document_id() {
        use crate::error::Error;
        let firestore_path_error =
            <firestore_path::DocumentId as std::str::FromStr>::from_str("").unwrap_err();
        let error = Error::invalid_document_id(firestore_path_error);
        assert_eq!(error.to_string(), "firestore error: invalid document id");
    }

    #[test]
    fn test_invalid_document_path() {
        use crate::error::Error;
        let firestore_path_error =
            <firestore_path::DocumentPath as std::str::FromStr>::from_str("").unwrap_err();
        let error = Error::invalid_document_path(firestore_path_error);
        assert_eq!(error.to_string(), "firestore error: invalid document path");
    }
}
