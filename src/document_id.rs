use crate::Error;

#[derive(Debug, thiserror::Error)]
#[error("document id error: {0}")]
struct E(firestore_path::Error);

impl From<E> for Error {
    fn from(e: E) -> Self {
        Self::from_source(Box::new(e))
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct DocumentId(firestore_path::DocumentId);

impl DocumentId {
    pub(crate) fn from_document_id(document_id: firestore_path::DocumentId) -> Self {
        Self(document_id)
    }

    pub(crate) fn generate() -> Self {
        let s = rand::distr::SampleString::sample_string(
            &rand::distr::Alphanumeric,
            &mut rand::rand_core::UnwrapErr(rand::rngs::SysRng),
            20,
        );
        <Self as std::str::FromStr>::from_str(&s).expect("generated document id should be valid")
    }
}

impl From<DocumentId> for firestore_path::DocumentId {
    fn from(document_id: DocumentId) -> Self {
        document_id.0
    }
}

impl From<firestore_path::DocumentId> for DocumentId {
    fn from(document_id: firestore_path::DocumentId) -> Self {
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
    fn test_clone() -> anyhow::Result<()> {
        use crate::document_id::DocumentId;
        use std::str::FromStr as _;
        let document_id = DocumentId::from_str("roomA")?;
        let cloned = document_id.clone();
        assert_eq!(cloned.to_string(), "roomA");
        Ok(())
    }

    #[test]
    fn test_debug() -> anyhow::Result<()> {
        use crate::document_id::DocumentId;
        use std::str::FromStr as _;
        let document_id = DocumentId::from_str("roomA")?;
        assert_eq!(
            format!("{:?}", document_id),
            "DocumentId(DocumentId(\"roomA\"))"
        );
        Ok(())
    }

    #[test]
    fn test_eq() -> anyhow::Result<()> {
        use crate::document_id::DocumentId;
        use std::str::FromStr as _;
        let d1 = DocumentId::from_str("roomA")?;
        let d2 = DocumentId::from_str("roomA")?;
        let d3 = DocumentId::from_str("roomB")?;
        assert_eq!(d1, d2);
        assert_ne!(d1, d3);
        Ok(())
    }

    #[test]
    fn test_hash() -> anyhow::Result<()> {
        use crate::document_id::DocumentId;
        use std::collections::HashSet;
        use std::str::FromStr as _;
        let d1 = DocumentId::from_str("roomA")?;
        let d2 = DocumentId::from_str("roomA")?;
        let d3 = DocumentId::from_str("roomB")?;
        let mut set = HashSet::new();
        set.insert(d1);
        assert!(!set.insert(d2));
        assert!(set.insert(d3));
        Ok(())
    }

    #[test]
    fn test_ord() -> anyhow::Result<()> {
        use crate::document_id::DocumentId;
        use std::str::FromStr as _;
        let d1 = DocumentId::from_str("roomA")?;
        let d2 = DocumentId::from_str("roomB")?;
        assert!(d1 < d2);
        assert!(d2 > d1);
        assert_eq!(d1.cmp(&d1), std::cmp::Ordering::Equal);
        Ok(())
    }

    #[test]
    fn test_display() -> anyhow::Result<()> {
        use crate::document_id::DocumentId;
        use std::str::FromStr as _;
        let document_id = DocumentId::from_str("roomA")?;
        assert_eq!(document_id.to_string(), "roomA");
        Ok(())
    }

    #[test]
    fn test_from_document_id() -> anyhow::Result<()> {
        use crate::document_id::DocumentId;
        use std::str::FromStr as _;
        let inner = firestore_path::DocumentId::from_str("roomA")?;
        let document_id = DocumentId::from_document_id(inner);
        assert_eq!(document_id.to_string(), "roomA");
        Ok(())
    }

    #[test]
    fn test_from_firestore_path_document_id() -> anyhow::Result<()> {
        use crate::document_id::DocumentId;
        use std::str::FromStr as _;
        let inner = firestore_path::DocumentId::from_str("roomA")?;
        let document_id = DocumentId::from(inner);
        assert_eq!(document_id.to_string(), "roomA");
        Ok(())
    }

    #[test]
    fn test_from_self_for_firestore_path_document_id() -> anyhow::Result<()> {
        use crate::document_id::DocumentId;
        use std::str::FromStr as _;
        let document_id = DocumentId::from_str("roomA")?;
        let inner = firestore_path::DocumentId::from(document_id);
        assert_eq!(inner.to_string(), "roomA");
        Ok(())
    }

    #[test]
    fn test_from_str() -> anyhow::Result<()> {
        use crate::document_id::DocumentId;
        use std::str::FromStr as _;
        assert!(DocumentId::from_str("roomA").is_ok());
        Ok(())
    }

    #[test]
    fn test_from_str_error() {
        use crate::document_id::DocumentId;
        use std::str::FromStr as _;
        assert!(DocumentId::from_str("").is_err());
    }

    #[test]
    fn test_generate() -> anyhow::Result<()> {
        use crate::document_id::DocumentId;
        let document_id = DocumentId::generate();
        assert_eq!(document_id.to_string().len(), 20);

        let mut chars = std::collections::BTreeSet::new();
        let mut set = std::collections::BTreeSet::new();
        for _ in 0..100 {
            let document_id = DocumentId::generate();
            chars.extend(document_id.to_string().chars());
            assert!(set.insert(document_id));
        }
        Ok(())
    }
}
