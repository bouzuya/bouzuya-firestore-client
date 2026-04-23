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
    let docs = query_snapshot.docs();
    assert_eq!(docs.len(), 1);
    let doc = &docs[0];
    let got: HashMap<String, String> = doc.data()?;
    assert_eq!(got.get("key").map(String::as_str), Some("value"));
    Ok(())
}
