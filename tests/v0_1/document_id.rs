#[test]
fn test_document_id_import() {
    use bouzuya_firestore_client::DocumentId;
    let _: Option<DocumentId> = None;
}

#[test]
fn test_document_id_from_str() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::DocumentId;
    use std::str::FromStr as _;
    assert!(DocumentId::from_str("roomA").is_ok());
    Ok(())
}

#[test]
fn test_document_id_from_str_error() {
    use bouzuya_firestore_client::DocumentId;
    use std::str::FromStr as _;
    assert!(DocumentId::from_str("").is_err());
}

#[test]
fn test_document_id_display() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::DocumentId;
    use std::str::FromStr as _;
    let document_id = DocumentId::from_str("roomA")?;
    assert_eq!(document_id.to_string(), "roomA");
    Ok(())
}
