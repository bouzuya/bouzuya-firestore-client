use std::str::FromStr as _;

use crate::Error;
use crate::FieldPath;

/// A conversion into a [`FieldPath`].
///
/// This trait is used as a bound on APIs that accept a field reference
/// — such as [`Filter::where`](crate::Filter::r#where) — so callers can
/// pass a [`FieldPath`] directly or a string to be parsed into one. It is
/// implemented for [`FieldPath`], `&str`, and `String`.
///
/// The trait is sealed: it cannot be implemented for types outside this
/// crate.
pub trait IntoFieldPath: crate::private::Sealed {
    /// Converts this value into a [`FieldPath`].
    ///
    /// This is the conversion behind APIs that accept
    /// `impl IntoFieldPath` (such as
    /// [`Filter::where`](crate::Filter::r#where)), letting callers pass a
    /// [`FieldPath`] directly or a string to be parsed. It is implemented
    /// for [`FieldPath`] (returned as-is), `&str`, and `String` (each
    /// parsed via [`FieldPath`]'s [`FromStr`](std::str::FromStr)).
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] when `self` is a string that is not a valid
    /// field path; converting a [`FieldPath`] never fails.
    fn into_field_path(self) -> Result<FieldPath, Error>;
}

impl crate::private::Sealed for FieldPath {}

impl IntoFieldPath for FieldPath {
    fn into_field_path(self) -> Result<FieldPath, Error> {
        Ok(self)
    }
}

impl crate::private::Sealed for &str {}

impl IntoFieldPath for &str {
    fn into_field_path(self) -> Result<FieldPath, Error> {
        FieldPath::from_str(self)
    }
}

impl crate::private::Sealed for String {}

impl IntoFieldPath for String {
    fn into_field_path(self) -> Result<FieldPath, Error> {
        FieldPath::from_str(&self)
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
