// since v3.1
#[tokio::test]
async fn test_firestore_collection_group() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let _collection_group = firestore.collection_group("rooms")?;
    // NOTE: can't test CollectionGroup::collection_id() because it's private
    Ok(())
}

// since v3.1
#[tokio::test]
async fn test_firestore_collection_group_invalid_collection_id()
-> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    assert!(firestore.collection_group("rooms/123/messages").is_err());
    Ok(())
}
