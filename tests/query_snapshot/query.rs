// since v2.1
#[tokio::test]
#[serial_test::serial]
async fn test_query_snapshot_query() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Query;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("test-query-snapshot-query")?;
    let query = collection_ref.limit(1);
    let query_snapshot = query.get().await?;
    let _: Query = query_snapshot.query();
    // FIXME: add assertion comparing query when Query implements PartialEq
    Ok(())
}
