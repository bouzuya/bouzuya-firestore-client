use crate::Error;
use crate::Filter;

pub(crate) trait IntoFilter {
    #[allow(dead_code)]
    fn into_filter(self) -> Result<Filter, Error>;
}

impl IntoFilter for Filter {
    fn into_filter(self) -> Result<Filter, Error> {
        Ok(self)
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
}
