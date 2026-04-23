// since v0.1
#[tokio::test]
async fn test_collection_reference_doc() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    let document_reference = collection_reference.doc("roomA")?;
    assert_eq!(document_reference.id().to_string(), "roomA");
    Ok(())
}
