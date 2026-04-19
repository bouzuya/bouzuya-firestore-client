// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_write_result_write_time() -> anyhow::Result<()> {
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
    let data: HashMap<String, String> = HashMap::new();
    let write_result = document_ref.create(data).await?;
    let write_time: Timestamp = write_result.write_time();
    assert!(write_time.to_millis() > 0);
    Ok(())
}
