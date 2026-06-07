// since v3.1 -> v4 (breaking change)
#[tokio::test]
async fn test_collection_group_firestore() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_group = firestore.collection_group("rooms")?;
    let actual: Firestore = collection_group.firestore();
    assert_eq!(actual, firestore);
    Ok(())
}
