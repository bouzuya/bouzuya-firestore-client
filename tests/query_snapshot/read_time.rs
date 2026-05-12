// since v3.2
#[tokio::test]
#[serial_test::serial]
async fn test_query_snapshot_read_time() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Timestamp;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("query_snapshot_read_time")?;
    let before = i64::try_from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis(),
    )?;
    let query_snapshot = collection_reference.get().await?;
    let after = i64::try_from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis(),
    )?;
    let read_time: Timestamp = query_snapshot.read_time();
    assert!((before..=after).contains(&read_time.to_millis()));
    Ok(())
}
