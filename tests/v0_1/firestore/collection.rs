#[tokio::test]
async fn test_firestore_collection() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("rooms")?;
    assert_eq!(collection_ref.id().to_string(), "rooms");
    Ok(())
}
