#[test]
fn test_collection_reference_import() {
    use bouzuya_firestore_client::CollectionReference;
    let _: Option<CollectionReference> = None;
}

#[test]
fn test_collection_reference_doc() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::CollectionId;
    use bouzuya_firestore_client::DocumentId;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::str::FromStr as _;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection(CollectionId::from_str("rooms")?);
    let document_ref = collection_ref.doc(DocumentId::from_str("roomA")?);
    assert_eq!(document_ref.id().to_string(), "roomA");
    Ok(())
}

#[test]
fn test_collection_reference_id() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::CollectionId;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::str::FromStr as _;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_id = CollectionId::from_str("rooms")?;
    let collection_ref = firestore.collection(collection_id);
    assert_eq!(collection_ref.id().to_string(), "rooms");
    Ok(())
}

#[test]
fn test_collection_reference_parent() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::CollectionId;
    use bouzuya_firestore_client::CollectionPath;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::str::FromStr as _;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection(CollectionId::from_str("rooms")?);
    let parent = collection_ref.parent();
    assert!(parent.is_none());
    let collection_ref = firestore.collection(CollectionPath::from_str("rooms/roomA/messages")?);
    let parent = collection_ref
        .parent()
        .expect("parent collection reference should exist");
    assert_eq!(parent.id().to_string(), "roomA");
    Ok(())
}

#[test]
fn test_collection_reference_path() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::CollectionId;
    use bouzuya_firestore_client::CollectionPath;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::str::FromStr as _;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection(CollectionId::from_str("rooms")?);
    let path = collection_ref.path();
    assert_eq!(path.to_string(), "rooms");
    let collection_ref = firestore.collection(CollectionPath::from_str("rooms/roomA/messages")?);
    let path = collection_ref.path();
    assert_eq!(path.to_string(), "rooms/roomA/messages");
    Ok(())
}
