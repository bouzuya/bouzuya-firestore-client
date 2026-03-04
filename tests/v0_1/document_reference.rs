#[tokio::test]
async fn test_document_reference_clone() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::DocumentReference;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let cloned = document_ref.clone();
    assert_eq!(cloned.id().to_string(), "roomA");

    fn assert_fn<T: Clone>() {}
    assert_fn::<DocumentReference>();
    Ok(())
}

#[tokio::test]
async fn test_document_reference_get() -> Result<(), bouzuya_firestore_client::Error> {
    // FIXME: add tests when the document exists and does not exist
    Ok(())
}

#[test]
fn test_document_reference_import() {
    use bouzuya_firestore_client::DocumentReference;
    let _: Option<DocumentReference> = None;
}

#[tokio::test]
async fn test_document_reference_collection() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::CollectionId;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::str::FromStr as _;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let collection_ref = document_ref.collection(CollectionId::from_str("messages")?);
    assert_eq!(collection_ref.path().to_string(), "rooms/roomA/messages");
    Ok(())
}

#[tokio::test]
async fn test_document_reference_id() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let document_id = document_ref.id();
    assert_eq!(document_id.to_string(), "roomA");
    Ok(())
}

#[tokio::test]
async fn test_document_reference_parent() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let parent = document_ref.parent();
    assert_eq!(parent.path().to_string(), "rooms");
    let document_ref = firestore.doc("rooms/roomA/messages/message1")?;
    let parent = document_ref.parent();
    assert_eq!(parent.path().to_string(), "rooms/roomA/messages");
    Ok(())
}

#[tokio::test]
async fn test_document_reference_path() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let path = document_ref.path();
    assert_eq!(path.to_string(), "rooms/roomA");
    let document_ref = firestore.doc("rooms/roomA/messages/message1")?;
    let path = document_ref.path();
    assert_eq!(path.to_string(), "rooms/roomA/messages/message1");
    Ok(())
}
