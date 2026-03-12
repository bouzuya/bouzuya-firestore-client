#[derive(Default)]
pub struct SetOptions {
    pub merge: Option<bool>,
    // TODO: FieldPath support
    pub merge_fields: Option<Vec<String>>,
}
