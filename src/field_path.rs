use crate::Error;

#[derive(Eq, PartialEq)]
pub struct FieldPath {
    segments: Vec<String>,
}

fn is_simple_segment(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        None => false,
        Some(c) => {
            (c.is_ascii_alphabetic() || c == '_')
                && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        }
    }
}

impl std::str::FromStr for FieldPath {
    type Err = Error;

    /// Parses a textual field path.
    ///
    /// The textual form is the one produced by
    /// [`Display`](std::fmt::Display): one or more segments joined by `.`.
    /// An *unquoted* segment must match `[A-Za-z_][A-Za-z0-9_]*` (an ASCII
    /// letter or `_` followed by ASCII letters, digits, or `_`). A *quoted*
    /// segment is wrapped in backticks and may contain any character;
    /// inside it, `` \` `` denotes a literal backtick and `\\` denotes a
    /// literal backslash.
    ///
    /// To construct a `FieldPath` from segments programmatically without
    /// parsing, use [`FieldPath::new`]; for a reference to the document's
    /// ID, use [`FieldPath::document_id`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] when:
    ///
    /// - a backtick-quoted segment is not closed,
    /// - an invalid escape (anything other than `` \` `` or `\\`) appears
    ///   inside a quoted segment,
    /// - a quoted segment is not followed by `.` or end of input,
    /// - an unquoted segment does not match the form above.
    ///
    /// # Examples
    ///
    /// ```
    /// use bouzuya_firestore_client::FieldPath;
    ///
    /// let nested: FieldPath = "user.name".parse()?;
    /// assert_eq!(nested.to_string(), "user.name");
    ///
    /// let quoted: FieldPath = "`x&y`".parse()?;
    /// assert_eq!(quoted.to_string(), "`x&y`");
    ///
    /// assert!("1foo".parse::<FieldPath>().is_err());
    /// # Ok::<(), bouzuya_firestore_client::Error>(())
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = Vec::new();
        let mut chars = s.chars().peekable();
        loop {
            match chars.peek().copied() {
                None => break,
                Some('`') => {
                    chars.next();
                    let mut seg = String::new();
                    loop {
                        match chars.next() {
                            None => {
                                return Err(Error::custom("unclosed backtick in field path"));
                            }
                            Some('`') => break,
                            Some('\\') => match chars.next() {
                                Some('`') => seg.push('`'),
                                Some('\\') => seg.push('\\'),
                                _ => {
                                    return Err(Error::custom("invalid escape in field path"));
                                }
                            },
                            Some(c) => seg.push(c),
                        }
                    }
                    segments.push(seg);
                    match chars.next() {
                        None | Some('.') => {}
                        Some(_) => {
                            return Err(Error::custom("expected '.' after quoted segment"));
                        }
                    }
                }
                _ => {
                    let mut seg = String::new();
                    while let Some(&c) = chars.peek() {
                        if c == '.' {
                            break;
                        }
                        seg.push(chars.next().unwrap());
                    }
                    if !is_simple_segment(&seg) {
                        return Err(Error::custom("invalid unquoted segment in field path"));
                    }
                    segments.push(seg);
                    if chars.peek() == Some(&'.') {
                        chars.next();
                    }
                }
            }
        }
        Ok(Self { segments })
    }
}

impl std::fmt::Display for FieldPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: Vec<String> = self
            .segments
            .iter()
            .map(|s| {
                if is_simple_segment(s) {
                    s.clone()
                } else {
                    format!("`{}`", s.replace('\\', "\\\\").replace('`', "\\`"))
                }
            })
            .collect();
        write!(f, "{}", parts.join("."))
    }
}

impl FieldPath {
    /// Returns a special [`FieldPath`] value that refers to a document's ID.
    ///
    /// Firestore exposes a document's identity as the reserved field
    /// `__name__`. Use this where a regular field path is accepted — for
    /// example, to sort or filter a query by the document ID — rather than
    /// constructing a `FieldPath` containing the literal `"__name__"`
    /// yourself.
    ///
    /// # Examples
    ///
    /// ```
    /// use bouzuya_firestore_client::FieldPath;
    ///
    /// let field_path = FieldPath::document_id();
    /// # let _ = field_path;
    /// ```
    pub fn document_id() -> Self {
        Self {
            segments: vec!["__name__".to_string()],
        }
    }

    /// Creates a [`FieldPath`] from the given field name segments.
    ///
    /// Each segment is one field name; passing more than one segment points
    /// at a nested field in the document (e.g. `["user", "name"]` refers to
    /// the `name` field of the `user` map). Segments may contain any
    /// character — special characters that are not valid in an unquoted
    /// field path (anything besides ASCII letters, digits, and `_`, or a
    /// leading digit) are automatically backtick-quoted by
    /// [`Display`](std::fmt::Display); see also [`FromStr`](std::str::FromStr).
    ///
    /// To refer to a document's ID instead of a stored field, use
    /// [`FieldPath::document_id`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bouzuya_firestore_client::FieldPath;
    ///
    /// let top_level = FieldPath::new(["age"])?;
    /// assert_eq!(top_level.to_string(), "age");
    ///
    /// let nested = FieldPath::new(["user", "name"])?;
    /// assert_eq!(nested.to_string(), "user.name");
    ///
    /// let needs_quoting = FieldPath::new(["x&y"])?;
    /// assert_eq!(needs_quoting.to_string(), "`x&y`");
    /// # Ok::<(), bouzuya_firestore_client::Error>(())
    /// ```
    pub fn new(segments: impl IntoIterator<Item = impl Into<String>>) -> Result<Self, Error> {
        Ok(Self {
            segments: segments.into_iter().map(Into::into).collect(),
        })
    }
}

impl FieldPath {
    pub(crate) fn into_structured_query_field_path(self) -> firestore_structured_query::FieldPath {
        firestore_structured_query::FieldPath::raw(self.to_string())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_into_structured_query_field_path() -> anyhow::Result<()> {
        use super::FieldPath;
        let field_path = FieldPath::new(["a"])?;
        assert_eq!(
            field_path.into_structured_query_field_path(),
            firestore_structured_query::FieldPath::raw("a")
        );
        Ok(())
    }
}
