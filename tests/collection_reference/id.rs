// since v0.1
#[tokio::test]
async fn test_collection_reference_id() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    assert_eq!(collection_reference.id().to_string(), "rooms");
    Ok(())
}
