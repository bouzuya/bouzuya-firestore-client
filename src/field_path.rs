use crate::Error;

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
    pub fn document_id() -> Self {
        Self {
            segments: vec!["__name__".to_string()],
        }
    }

    pub fn new(segments: impl IntoIterator<Item = impl Into<String>>) -> Result<Self, Error> {
        Ok(Self {
            segments: segments.into_iter().map(Into::into).collect(),
        })
    }
}

impl FieldPath {
    #[allow(dead_code)]
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
