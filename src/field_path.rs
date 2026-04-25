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

    pub fn new(
        segments: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            segments: segments.into_iter().map(Into::into).collect(),
        })
    }
}
