#[test]
fn test_collection_path_import() {
    use bouzuya_firestore_client::CollectionPath;
    let _: Option<CollectionPath> = None;
}

#[test]
fn test_collection_path_from_collection_id() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::CollectionId;
    use bouzuya_firestore_client::CollectionPath;
    use std::str::FromStr as _;
    let collection_id = CollectionId::from_str("rooms")?;
    let collection_path = CollectionPath::from(collection_id);
    assert_eq!(collection_path.to_string(), "rooms");
    Ok(())
}

#[test]
fn test_collection_path_from_str() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::CollectionPath;
    use std::str::FromStr as _;
    let collection_path = CollectionPath::from_str("rooms/roomA/messages")?;
    assert_eq!(collection_path.to_string(), "rooms/roomA/messages");
    Ok(())
}

#[test]
fn test_collection_path_from_str_error() {
    use bouzuya_firestore_client::CollectionPath;
    use std::str::FromStr as _;
    assert!(CollectionPath::from_str("").is_err());
}

#[test]
fn test_collection_path_display() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::CollectionPath;
    use std::str::FromStr as _;
    let collection_path = CollectionPath::from_str("rooms/roomA/messages")?;
    assert_eq!(collection_path.to_string(), "rooms/roomA/messages");
    Ok(())
}
