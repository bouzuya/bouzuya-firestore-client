use crate::Error;
use crate::IntoFieldPath;

pub struct Filter(firestore_structured_query::Filter);

impl Filter {
    pub fn and<I>(filters: I) -> Self
    where
        I: IntoIterator<Item = Filter>,
    {
        Self(firestore_structured_query::Filter::and(
            filters.into_iter().map(|f| f.0),
        ))
    }

    pub fn or<I>(filters: I) -> Self
    where
        I: IntoIterator<Item = Filter>,
    {
        Self(firestore_structured_query::Filter::or(
            filters.into_iter().map(|f| f.0),
        ))
    }

    #[allow(private_bounds)]
    pub fn r#where(
        field_path: impl IntoFieldPath,
        op: &str,
        value: impl serde::Serialize,
    ) -> Result<Self, Error> {
        let field_path = field_path
            .into_field_path()?
            .into_structured_query_field_path();
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

impl Filter {
    pub(crate) fn into_inner(self) -> firestore_structured_query::Filter {
        self.0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_into_inner() -> anyhow::Result<()> {
        use crate::FieldPath;
        use crate::Filter;
        let f1 = Filter::r#where(FieldPath::new(["age"])?, "==", 30_i64)?.into_inner();
        let f2 = Filter::r#where(FieldPath::new(["age"])?, "==", 30_i64)?.into_inner();
        assert_eq!(f1, f2);
        Ok(())
    }
}
