// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_document_snapshot_update_time() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Timestamp;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_reference = firestore.doc(format!("rooms/{}", id))?;

    // non-existing document has no update_time
    let snapshot = document_reference.get().await?;
    assert_eq!(snapshot.update_time(), None);

    // existing document has update_time
    document_reference
        .create(HashMap::<String, String>::new())
        .await?;
    let snapshot = document_reference.get().await?;
    let update_time: Option<Timestamp> = snapshot.update_time();
    assert!(update_time.is_some_and(|t| t.to_millis() > 0));

    Ok(())
}
