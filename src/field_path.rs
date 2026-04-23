pub struct FieldPath {
    #[expect(dead_code)]
    field: Vec<String>,
}

impl FieldPath {
    pub fn new(fields: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            field: fields.into_iter().map(Into::into).collect(),
        }
    }
}
