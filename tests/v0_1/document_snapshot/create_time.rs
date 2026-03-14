#[tokio::test]
#[serial_test::serial]
async fn test_document_snapshot_create_time() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Timestamp;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_ref = firestore.doc(format!("rooms/{}", id))?;

    // non-existing document has no create_time
    let snapshot = document_ref.get().await?;
    assert_eq!(snapshot.create_time(), None);

    // existing document has create_time
    document_ref
        .create(HashMap::<String, String>::new())
        .await?;
    let snapshot = document_ref.get().await?;
    let create_time: Option<Timestamp> = snapshot.create_time();
    assert!(create_time.is_some_and(|t| t.to_millis() > 0));

    Ok(())
}
