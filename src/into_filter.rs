use crate::Error;
use crate::Filter;
use crate::IntoFieldPath;

pub trait IntoFilter: crate::private::Sealed {
    /// Converts this value into a [`Filter`].
    ///
    /// This is the conversion behind APIs that accept `impl IntoFilter`
    /// (such as the `where` method of a query), letting callers pass a
    /// [`Filter`] directly or a `(field_path, op, value)` tuple as
    /// shorthand. It is implemented for [`Filter`] (returned as-is) and for
    /// 3-tuples `(P, &str, V)` where `P: IntoFieldPath` and
    /// `V: serde::Serialize`; a tuple is forwarded to
    /// [`Filter::where`](crate::Filter::r#where).
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] when converting a tuple fails for any of the
    /// reasons [`Filter::where`](crate::Filter::r#where) can fail (invalid
    /// field path, unsupported operator, or unserializable value);
    /// converting a [`Filter`] never fails.
    fn into_filter(self) -> Result<Filter, Error>;
}

impl crate::private::Sealed for Filter {}

impl IntoFilter for Filter {
    fn into_filter(self) -> Result<Filter, Error> {
        Ok(self)
    }
}

impl<P, V> crate::private::Sealed for (P, &str, V)
where
    P: IntoFieldPath,
    V: serde::Serialize,
{
}

impl<P, V> IntoFilter for (P, &str, V)
where
    P: IntoFieldPath,
    V: serde::Serialize,
{
    fn into_filter(self) -> Result<Filter, Error> {
        Filter::r#where(self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_filter_into_filter() -> anyhow::Result<()> {
        use crate::Filter;
        use crate::IntoFilter;
        let filter = Filter::r#where("k", "==", 1_i64)?;
        let _: Filter = filter.into_filter()?;
        Ok(())
    }

    #[test]
    fn test_tuple_into_filter() -> anyhow::Result<()> {
        use crate::Filter;
        use crate::IntoFilter;
        let _: Filter = ("k", "==", 1_i64).into_filter()?;
        Ok(())
    }
}
