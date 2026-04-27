// since v2.2
#[test]
fn test_filter_where_() {
    fn _with_string(f: String, op: &str, v: i64) {
        use bouzuya_firestore_client::Filter;
        let _ = Filter::r#where(f, op, v);
    }
    fn _with_field_path(f: bouzuya_firestore_client::FieldPath, op: &str, v: i64) {
        use bouzuya_firestore_client::Filter;
        let _ = Filter::r#where(f, op, v);
    }
}

// since v2.2
#[test]
fn test_where_less_than() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    use bouzuya_firestore_client::Filter;
    let fp = FieldPath::new(["age"])?;
    assert!(Filter::r#where(fp, "<", 30_i64).is_ok());
    Ok(())
}

// since v2.2
#[test]
fn test_where_less_than_or_equal() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    use bouzuya_firestore_client::Filter;
    let fp = FieldPath::new(["age"])?;
    assert!(Filter::r#where(fp, "<=", 30_i64).is_ok());
    Ok(())
}

// since v2.2
#[test]
fn test_where_equal() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    use bouzuya_firestore_client::Filter;
    let fp = FieldPath::new(["age"])?;
    assert!(Filter::r#where(fp, "==", 30_i64).is_ok());
    Ok(())
}

// since v2.2
#[test]
fn test_where_not_equal() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    use bouzuya_firestore_client::Filter;
    let fp = FieldPath::new(["age"])?;
    assert!(Filter::r#where(fp, "!=", 30_i64).is_ok());
    Ok(())
}

// since v2.2
#[test]
fn test_where_greater_than_or_equal() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    use bouzuya_firestore_client::Filter;
    let fp = FieldPath::new(["age"])?;
    assert!(Filter::r#where(fp, ">=", 30_i64).is_ok());
    Ok(())
}

// since v2.2
#[test]
fn test_where_greater_than() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    use bouzuya_firestore_client::Filter;
    let fp = FieldPath::new(["age"])?;
    assert!(Filter::r#where(fp, ">", 30_i64).is_ok());
    Ok(())
}

// since v2.2
#[test]
fn test_where_array_contains() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    use bouzuya_firestore_client::Filter;
    let fp = FieldPath::new(["tags"])?;
    assert!(Filter::r#where(fp, "array-contains", "rust").is_ok());
    Ok(())
}

// since v2.2
#[test]
fn test_where_in() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    use bouzuya_firestore_client::Filter;
    let fp = FieldPath::new(["age"])?;
    assert!(Filter::r#where(fp, "in", vec![20_i64, 30_i64]).is_ok());
    Ok(())
}

// since v2.2
#[test]
fn test_where_not_in() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    use bouzuya_firestore_client::Filter;
    let fp = FieldPath::new(["age"])?;
    assert!(Filter::r#where(fp, "not-in", vec![20_i64, 30_i64]).is_ok());
    Ok(())
}

// since v2.2
#[test]
fn test_where_array_contains_any() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    use bouzuya_firestore_client::Filter;
    let fp = FieldPath::new(["tags"])?;
    assert!(Filter::r#where(fp, "array-contains-any", vec!["rust", "go"]).is_ok());
    Ok(())
}

// since v2.2
#[test]
fn test_where_with_string_field() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Filter;
    assert!(Filter::r#where("age".to_string(), "==", 30_i64).is_ok());
    Ok(())
}

// since v2.2
#[test]
fn test_where_invalid_op() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    use bouzuya_firestore_client::Filter;
    let fp = FieldPath::new(["age"])?;
    assert!(Filter::r#where(fp, "invalid", 30_i64).is_err());
    Ok(())
}
