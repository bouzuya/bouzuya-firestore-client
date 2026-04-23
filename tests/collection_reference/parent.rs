// since v0.1
#[tokio::test]
async fn test_collection_reference_parent() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    let parent = collection_reference.parent();
    assert!(parent.is_none());
    let collection_reference = firestore.collection("rooms/roomA/messages")?;
    let parent = collection_reference
        .parent()
        .expect("parent collection reference should exist");
    assert_eq!(parent.id().to_string(), "roomA");
    Ok(())
}
