#[test]
fn test_document_path_import() {
    use bouzuya_firestore_client::DocumentPath;
    let _: Option<DocumentPath> = None;
}

#[test]
fn test_document_path_from_str() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::DocumentPath;
    use std::str::FromStr as _;
    assert!(DocumentPath::from_str("rooms/roomA").is_ok());
    Ok(())
}

#[test]
fn test_document_path_from_str_error() {
    use bouzuya_firestore_client::DocumentPath;
    use std::str::FromStr as _;
    assert!(DocumentPath::from_str("").is_err());
}

#[test]
fn test_document_path_display() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::DocumentPath;
    use std::str::FromStr as _;
    let document_path = DocumentPath::from_str("rooms/roomA")?;
    assert_eq!(document_path.to_string(), "rooms/roomA");
    Ok(())
}
