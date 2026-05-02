use crate::Error;
use crate::Filter;
use crate::IntoFieldPath;

pub(crate) trait IntoFilter {
    #[allow(dead_code)]
    fn into_filter(self) -> Result<Filter, Error>;
}

impl IntoFilter for Filter {
    fn into_filter(self) -> Result<Filter, Error> {
        Ok(self)
    }
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
