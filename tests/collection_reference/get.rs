// since v2.1
#[tokio::test]
#[serial_test::serial]
async fn test_collection_reference_get() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::QuerySnapshot;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    collection_reference
        .add(HashMap::<String, String>::new())
        .await?;
    let query_snapshot: QuerySnapshot = collection_reference.get().await?;
    assert!(!query_snapshot.empty());
    for snapshot in query_snapshot {
        assert!(snapshot.exists());
    }
    Ok(())
}
