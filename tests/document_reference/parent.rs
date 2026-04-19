// since v0.1
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
