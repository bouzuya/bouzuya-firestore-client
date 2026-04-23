// since v0.1
#[tokio::test]
async fn test_document_reference_path() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_reference = firestore.doc("rooms/roomA")?;
    let path = document_reference.path();
    assert_eq!(path.to_string(), "rooms/roomA");
    let document_reference = firestore.doc("rooms/roomA/messages/message1")?;
    let path = document_reference.path();
    assert_eq!(path.to_string(), "rooms/roomA/messages/message1");
    Ok(())
}
