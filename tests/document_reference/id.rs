// since v0.1
#[tokio::test]
async fn test_document_reference_id() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_reference = firestore.doc("rooms/roomA")?;
    let document_id = document_reference.id();
    assert_eq!(document_id.to_string(), "roomA");
    Ok(())
}
