#[tokio::test]
async fn test_collection_reference_doc() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("rooms")?;
    let document_ref = collection_ref.doc("roomA")?;
    assert_eq!(document_ref.id().to_string(), "roomA");
    Ok(())
}
