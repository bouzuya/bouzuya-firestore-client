// since v0.1
#[tokio::test]
async fn test_document_reference_collection() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_reference = firestore.doc("rooms/roomA")?;
    let collection_reference = document_reference.collection("messages")?;
    assert_eq!(
        collection_reference.path().to_string(),
        "rooms/roomA/messages"
    );
    Ok(())
}
