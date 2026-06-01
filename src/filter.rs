use crate::Error;
use crate::IntoFieldPath;

/// A condition used to constrain the documents a query returns.
///
/// A `Filter` is either a single field condition — built with
/// [`where`](Self::where), which tests a field against a value using an
/// operator — or a composite of other filters joined by
/// [`and`](Self::and) (conjunction) or [`or`](Self::or) (disjunction).
/// Composites nest arbitrarily, so any boolean combination of conditions
/// can be expressed.
///
/// Pass the resulting `Filter` to a query's `where` method to apply it.
///
/// # Examples
///
/// ```
/// use bouzuya_firestore_client::FieldPath;
/// use bouzuya_firestore_client::Filter;
///
/// let young = Filter::r#where(FieldPath::new(["age"])?, "<", 30_i64)?;
/// let named = Filter::r#where(FieldPath::new(["name"])?, "==", "Alice")?;
/// let _: Filter = Filter::and([young, named]);
/// # Ok::<(), bouzuya_firestore_client::Error>(())
/// ```
pub struct Filter(firestore_structured_query::Filter);

impl Filter {
    /// Combines the given filters with a logical AND.
    ///
    /// The returned [`Filter`] matches a document only when it matches
    /// *every* filter in `filters`. The arguments are typically field
    /// conditions built with [`Filter::where`](Self::where), but may also
    /// be nested [`and`](Self::and)/[`or`](Self::or) filters. See
    /// [`Filter::or`] for the disjunction.
    ///
    /// # Examples
    ///
    /// ```
    /// use bouzuya_firestore_client::FieldPath;
    /// use bouzuya_firestore_client::Filter;
    ///
    /// let f1 = Filter::r#where(FieldPath::new(["age"])?, "==", 30_i64)?;
    /// let f2 = Filter::r#where(FieldPath::new(["name"])?, "==", "Alice")?;
    /// let _: Filter = Filter::and([f1, f2]);
    /// # Ok::<(), bouzuya_firestore_client::Error>(())
    /// ```
    pub fn and<I>(filters: I) -> Self
    where
        I: IntoIterator<Item = Filter>,
    {
        Self(firestore_structured_query::Filter::and(
            filters.into_iter().map(|f| f.0),
        ))
    }

    /// Combines the given filters with a logical OR.
    ///
    /// The returned [`Filter`] matches a document when it matches *at least
    /// one* filter in `filters`. The arguments are typically field
    /// conditions built with [`Filter::where`](Self::where), but may also
    /// be nested [`and`](Self::and)/[`or`](Self::or) filters. See
    /// [`Filter::and`] for the conjunction.
    ///
    /// # Examples
    ///
    /// ```
    /// use bouzuya_firestore_client::FieldPath;
    /// use bouzuya_firestore_client::Filter;
    ///
    /// let f1 = Filter::r#where(FieldPath::new(["age"])?, "==", 30_i64)?;
    /// let f2 = Filter::r#where(FieldPath::new(["name"])?, "==", "Alice")?;
    /// let _: Filter = Filter::or([f1, f2]);
    /// # Ok::<(), bouzuya_firestore_client::Error>(())
    /// ```
    pub fn or<I>(filters: I) -> Self
    where
        I: IntoIterator<Item = Filter>,
    {
        Self(firestore_structured_query::Filter::or(
            filters.into_iter().map(|f| f.0),
        ))
    }

    /// Builds a single field-condition [`Filter`].
    ///
    /// The condition tests `field_path` against `value` using the operator
    /// `op`. `field_path` is anything implementing [`IntoFieldPath`] — a
    /// [`FieldPath`](crate::FieldPath), or a `&str`/`String` that is parsed
    /// as one (see [`FieldPath`](crate::FieldPath)'s
    /// [`FromStr`](std::str::FromStr)). `value` is any [`serde::Serialize`]
    /// value.
    ///
    /// `op` must be one of the following strings:
    ///
    /// - `"<"`, `"<="`, `"=="`, `"!="`, `">="`, `">"` — value comparisons,
    /// - `"array-contains"` — the array field contains `value`,
    /// - `"in"` — the field equals one of the elements of `value`,
    /// - `"not-in"` — the field equals none of the elements of `value`,
    /// - `"array-contains-any"` — the array field contains any element of
    ///   `value`.
    ///
    /// Combine several conditions with [`Filter::and`] or [`Filter::or`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] when `field_path` fails to parse, when `op` is
    /// not one of the strings listed above, when `value` cannot be
    /// serialized, or when the field/operator/value combination is rejected
    /// as a query condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use bouzuya_firestore_client::FieldPath;
    /// use bouzuya_firestore_client::Filter;
    ///
    /// let _ = Filter::r#where(FieldPath::new(["age"])?, "<", 30_i64)?;
    /// let _ = Filter::r#where("name", "==", "Alice")?;
    /// let _ = Filter::r#where("tags", "array-contains", "rust")?;
    ///
    /// assert!(Filter::r#where("age", "invalid", 30_i64).is_err());
    /// # Ok::<(), bouzuya_firestore_client::Error>(())
    /// ```
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
