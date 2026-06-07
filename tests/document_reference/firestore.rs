// since v0.1 -> v4 (breaking change)
#[tokio::test]
async fn test_document_reference_firestore() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_reference = firestore.doc("rooms/roomA")?;
    let actual: Firestore = document_reference.firestore();
    assert_eq!(actual, firestore);
    Ok(())
}
