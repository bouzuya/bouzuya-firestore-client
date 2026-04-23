// since v1.1
#[tokio::test]
async fn test_collection_reference_firestore() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    let _: &Firestore = collection_reference.firestore();
    Ok(())
}
