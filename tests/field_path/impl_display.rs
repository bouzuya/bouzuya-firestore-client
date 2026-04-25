// since v2.1
#[test]
fn test_simple_single_segment() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let fp = FieldPath::new(["a"])?;
    assert_eq!(fp.to_string(), "a");
    Ok(())
}

// since v2.1
#[test]
fn test_simple_multiple_segments() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let fp = FieldPath::new(["a", "b"])?;
    assert_eq!(fp.to_string(), "a.b");
    Ok(())
}

// since v2.1
#[test]
fn test_quoted_segment() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let fp = FieldPath::new(["x&y"])?;
    assert_eq!(fp.to_string(), "`x&y`");
    Ok(())
}

// since v2.1
#[test]
fn test_quoted_segment_with_backtick() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let fp = FieldPath::new(["a`b"])?;
    assert_eq!(fp.to_string(), r"`a\`b`");
    Ok(())
}

// since v2.1
#[test]
fn test_quoted_segment_with_backslash() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let fp = FieldPath::new([r"a\b"])?;
    assert_eq!(fp.to_string(), r"`a\\b`");
    Ok(())
}

// since v2.1
#[test]
fn test_mixed_simple_and_quoted_segments() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let fp = FieldPath::new(["foo", "x&y"])?;
    assert_eq!(fp.to_string(), "foo.`x&y`");
    Ok(())
}
