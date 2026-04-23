// since v2.1
#[tokio::test]
#[serial_test::serial]
async fn test_query_snapshot_query() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-query-snapshot-query")?;
    let query = collection_reference.limit(1);
    let query_snapshot = query.get().await?;
    assert_eq!(query_snapshot.query(), query);
    Ok(())
}
