// since v2.1
#[test]
fn test_field_path_new() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let _field_path = FieldPath::new(["a", "b"])?;
    // FIXME: field is private, cannot verify the value
    Ok(())
}
