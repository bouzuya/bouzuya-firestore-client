use crate::Error;
use crate::FieldPath;

pub(crate) trait IntoFieldPath {
    #[allow(dead_code)]
    fn into_field_path(self) -> Result<FieldPath, Error>;
}

impl IntoFieldPath for FieldPath {
    fn into_field_path(self) -> Result<FieldPath, Error> {
        Ok(self)
    }
}

impl IntoFieldPath for &str {
    fn into_field_path(self) -> Result<FieldPath, Error> {
        <FieldPath as std::str::FromStr>::from_str(self)
    }
}

impl IntoFieldPath for String {
    fn into_field_path(self) -> Result<FieldPath, Error> {
        <FieldPath as std::str::FromStr>::from_str(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::IntoFieldPath;

    #[test]
    fn test_field_path_into_field_path() -> anyhow::Result<()> {
        use crate::FieldPath;
        let fp = FieldPath::new(["age"])?;
        assert_eq!(fp.into_field_path()?.to_string(), "age");
        Ok(())
    }

    #[test]
    fn test_str_into_field_path() -> anyhow::Result<()> {
        use super::IntoFieldPath;
        assert_eq!("age".into_field_path()?.to_string(), "age");
        Ok(())
    }

    #[test]
    fn test_string_into_field_path() -> anyhow::Result<()> {
        assert_eq!("age".to_string().into_field_path()?.to_string(), "age");
        Ok(())
    }
}
