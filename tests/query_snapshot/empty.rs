// since v2.1
#[tokio::test]
#[serial_test::serial]
async fn test_query_snapshot_empty() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("query_snapshot_empty")?;
    let query_snapshot = collection_ref.get().await?;
    assert!(query_snapshot.empty());
    collection_ref.add(HashMap::<String, String>::new()).await?;
    let query_snapshot = collection_ref.get().await?;
    assert!(!query_snapshot.empty());
    Ok(())
}
