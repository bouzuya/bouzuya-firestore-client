#[tokio::test]
async fn test_document_reference_collection() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let collection_ref = document_ref.collection("messages")?;
    assert_eq!(collection_ref.path().to_string(), "rooms/roomA/messages");
    Ok(())
}
