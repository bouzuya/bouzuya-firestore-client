// since v2.1
#[tokio::test]
#[serial_test::serial]
async fn test_query_document_snapshot_create_time() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Timestamp;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let collection_ref = firestore.collection(format!("rooms/{}/items", id))?;
    collection_ref.add(HashMap::<String, String>::new()).await?;
    let query_snapshot = collection_ref.get().await?;
    let docs = query_snapshot.docs();
    assert_eq!(docs.len(), 1);
    let _create_time: Timestamp = docs[0].create_time();
    Ok(())
}
