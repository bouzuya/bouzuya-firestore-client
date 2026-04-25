// since v2.1
#[test]
fn test_field_path_document_id() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let _field_path = FieldPath::document_id();
    // FIXME: segments is private, cannot verify the value
    Ok(())
}
