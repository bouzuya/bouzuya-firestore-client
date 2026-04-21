// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_document_snapshot_data() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_ref = firestore.doc(format!("rooms/{}", id))?;

    // non-existing document has no data
    let snapshot = document_ref.get().await?;
    let data: Option<Result<HashMap<String, String>, _>> = snapshot.data();
    assert!(data.is_none());

    // existing document with data
    let mut fields = HashMap::new();
    fields.insert("key".to_string(), "value".to_string());
    document_ref.create(fields).await?;
    let snapshot = document_ref.get().await?;
    let data: HashMap<String, String> = snapshot
        .data::<HashMap<String, String>>()
        .ok_or(anyhow::anyhow!("data is None"))??;
    assert_eq!(data.get("key"), Some(&"value".to_string()));

    Ok(())
}
