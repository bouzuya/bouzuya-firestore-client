use crate::Error;
use crate::IntoFieldPath;

#[allow(dead_code)]
pub struct Filter(firestore_structured_query::Filter);

impl Filter {
    #[allow(private_bounds)]
    pub fn r#where(
        field_path: impl IntoFieldPath,
        op: &str,
        value: impl serde::Serialize,
    ) -> Result<Self, Error> {
        let field_path = field_path.into_field_path()?;
        let field_path = firestore_structured_query::FieldPath::raw(field_path.to_string());
        let value =
            serde_firestore_value::to_value(&value).map_err(|e| Error::from_source(Box::new(e)))?;
        match op {
            "<" => field_path.less_than(value),
            "<=" => field_path.less_than_or_equal(value),
            "==" => field_path.equal(value),
            "!=" => field_path.not_equal(value),
            ">=" => field_path.greater_than_or_equal(value),
            ">" => field_path.greater_than(value),
            "array-contains" => field_path.array_contains(value),
            "in" => field_path.r#in(value),
            "not-in" => field_path.not_in(value),
            "array-contains-any" => field_path.array_contains_any(value),
            _ => return Err(Error::custom(format!("unsupported operator: {}", op))),
        }
        .map(Self)
        .map_err(|e| Error::from_source(Box::new(e)))
    }
}
