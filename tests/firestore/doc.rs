// since v0.1
#[tokio::test]
async fn test_firestore_doc() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_reference = firestore.doc("rooms/roomA")?;
    assert_eq!(document_reference.id().to_string(), "roomA");
    Ok(())
}
