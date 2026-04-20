#[allow(clippy::manual_non_exhaustive)]
pub struct QueryDocumentSnapshot {
    _private: (),
}

impl QueryDocumentSnapshot {
    pub fn exists(&self) -> bool {
        true
    }
}
