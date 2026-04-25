pub struct FieldPath {
    #[expect(dead_code)]
    segments: Vec<String>,
}

impl FieldPath {
    pub fn document_id() -> Self {
        Self {
            segments: vec!["__name__".to_string()],
        }
    }

    pub fn new(
        segments: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            segments: segments.into_iter().map(Into::into).collect(),
        })
    }
}
