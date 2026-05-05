// since v3.1
#[tokio::test]
async fn test_collection_group_firestore() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_group = firestore.collection_group("rooms")?;
    let firestore_ref: &Firestore = collection_group.firestore();
    assert_eq!(firestore_ref, &firestore);
    Ok(())
}
