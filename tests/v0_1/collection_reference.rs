#[tokio::test]
async fn test_collection_reference_add() -> anyhow::Result<()> {
    use bouzuya_firestore_client::DocumentReference;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("rooms")?;
    let document_ref: DocumentReference =
        collection_ref.add(HashMap::<String, String>::new()).await?;
    assert!(document_ref.path().starts_with("rooms/"));
    Ok(())
}

#[test]
fn test_collection_reference_import() {
    use bouzuya_firestore_client::CollectionReference;
    let _: Option<CollectionReference> = None;
}

#[tokio::test]
async fn test_collection_reference_doc() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("rooms")?;
    let document_ref = collection_ref.doc("roomA")?;
    assert_eq!(document_ref.id().to_string(), "roomA");
    Ok(())
}

#[tokio::test]
async fn test_collection_reference_id() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("rooms")?;
    assert_eq!(collection_ref.id().to_string(), "rooms");
    Ok(())
}

#[tokio::test]
async fn test_collection_reference_parent() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("rooms")?;
    let parent = collection_ref.parent();
    assert!(parent.is_none());
    let collection_ref = firestore.collection("rooms/roomA/messages")?;
    let parent = collection_ref
        .parent()
        .expect("parent collection reference should exist");
    assert_eq!(parent.id().to_string(), "roomA");
    Ok(())
}

#[tokio::test]
async fn test_collection_reference_path() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("rooms")?;
    let path = collection_ref.path();
    assert_eq!(path.to_string(), "rooms");
    let collection_ref = firestore.collection("rooms/roomA/messages")?;
    let path = collection_ref.path();
    assert_eq!(path.to_string(), "rooms/roomA/messages");
    Ok(())
}
