#[derive(Debug, thiserror::Error)]
enum E {
    #[error("unknown")]
    Unknown(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[derive(Debug, thiserror::Error)]
#[error("firestore error")]
pub struct Error(#[source] E);

impl Error {
    pub(crate) fn from_source(source: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Self(E::Unknown(source))
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
        assert_eq!(error.to_string(), "firestore error");
    }
}
