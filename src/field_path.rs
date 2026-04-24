use crate::Error;

pub struct FieldPath;

impl FieldPath {
    pub fn new(_fields: Vec<String>) -> Result<Self, Error> {
        Ok(Self)
    }
}
