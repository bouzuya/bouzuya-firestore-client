// since v2.1
#[test]
fn test_simple_single_segment() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let fp: FieldPath = "a".parse()?;
    assert_eq!(fp.to_string(), "a");
    Ok(())
}

// since v2.1
#[test]
fn test_simple_multiple_segments() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let fp: FieldPath = "a.b".parse()?;
    assert_eq!(fp.to_string(), "a.b");
    Ok(())
}

// since v2.1
#[test]
fn test_quoted_segment() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let fp: FieldPath = "`x&y`".parse()?;
    assert_eq!(fp.to_string(), "`x&y`");
    Ok(())
}

// since v2.1
#[test]
fn test_quoted_segment_with_escaped_backtick() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let fp: FieldPath = r"`a\`b`".parse()?;
    assert_eq!(fp.to_string(), r"`a\`b`");
    Ok(())
}

// since v2.1
#[test]
fn test_quoted_segment_with_escaped_backslash() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let fp: FieldPath = r"`a\\b`".parse()?;
    assert_eq!(fp.to_string(), r"`a\\b`");
    Ok(())
}

// since v2.1
#[test]
fn test_mixed_simple_and_quoted_segments() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let fp: FieldPath = "foo.`x&y`".parse()?;
    assert_eq!(fp.to_string(), "foo.`x&y`");
    Ok(())
}

// since v2.1
#[test]
fn test_error_unclosed_backtick() {
    use bouzuya_firestore_client::FieldPath;
    assert!("`x&y".parse::<FieldPath>().is_err());
}

// since v2.1
#[test]
fn test_error_invalid_escape() {
    use bouzuya_firestore_client::FieldPath;
    assert!(r"`\a`".parse::<FieldPath>().is_err());
}

// since v2.1
#[test]
fn test_error_unquoted_non_simple_segment() {
    use bouzuya_firestore_client::FieldPath;
    assert!("x&y".parse::<FieldPath>().is_err());
}

// since v2.1
#[test]
fn test_error_unquoted_digit_start_segment() {
    use bouzuya_firestore_client::FieldPath;
    assert!("1foo".parse::<FieldPath>().is_err());
}

// since v2.1
#[test]
fn test_quoted_simple_segment() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let fp: FieldPath = "`foo`".parse()?;
    assert_eq!(fp.to_string(), "foo");
    Ok(())
}
