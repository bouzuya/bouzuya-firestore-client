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
