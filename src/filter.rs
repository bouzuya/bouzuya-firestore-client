use crate::Error;
use crate::IntoFieldPath;

pub struct Filter {
    _private: (),
}

impl Filter {
    #[allow(private_bounds)]
    pub fn r#where(
        _field: impl IntoFieldPath,
        _op: &str,
        _value: impl serde::Serialize,
    ) -> Result<Self, Error> {
        todo!()
    }
}
