// since v2.1
#[tokio::test]
#[serial_test::serial]
async fn test_query_snapshot_size() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("query_snapshot_size")?;
    let query_snapshot = collection_ref.get().await?;
    assert_eq!(query_snapshot.size(), 0);
    collection_ref.add(HashMap::<String, String>::new()).await?;
    let query_snapshot = collection_ref.get().await?;
    assert_eq!(query_snapshot.size(), 1);
    Ok(())
}
