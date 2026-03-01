#[test]
fn test_collection_id_import() {
    use bouzuya_firestore_client::CollectionId;
    let _: Option<CollectionId> = None;
}

#[test]
fn test_collection_id_from_str() {
    use bouzuya_firestore_client::CollectionId;
    use std::str::FromStr as _;
    assert!(CollectionId::from_str("chatrooms").is_ok());
}

#[test]
fn test_collection_id_from_str_error() {
    use bouzuya_firestore_client::CollectionId;
    use std::str::FromStr as _;
    assert!(CollectionId::from_str("").is_err());
}

#[test]
fn test_collection_id_display() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::CollectionId;
    use std::str::FromStr as _;
    let collection_id = CollectionId::from_str("chatrooms")?;
    assert_eq!(collection_id.to_string(), "chatrooms");
    Ok(())
}
