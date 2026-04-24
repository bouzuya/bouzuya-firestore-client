// since v2.1
#[test]
fn test_field_path_new() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let _ = FieldPath::new(vec!["a".to_string()])?;
    Ok(())
}
