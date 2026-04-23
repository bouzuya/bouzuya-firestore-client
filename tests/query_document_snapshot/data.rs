// since v2.1
#[tokio::test]
#[serial_test::serial]
async fn test_query_document_snapshot_data() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let collection_reference = firestore.collection(format!("rooms/{}/items", id))?;
    let data = HashMap::from([("key".to_string(), "value".to_string())]);
    collection_reference.add(data).await?;
    let query_snapshot = collection_reference.get().await?;
    let query_document_snapshots = query_snapshot.docs();
    assert_eq!(query_document_snapshots.len(), 1);
    let query_document_snapshot = &query_document_snapshots[0];
    let got: HashMap<String, String> = query_document_snapshot.data()?;
    assert_eq!(got.get("key").map(String::as_str), Some("value"));
    Ok(())
}
