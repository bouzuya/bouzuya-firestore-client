// since v2.1
#[tokio::test]
#[serial_test::serial]
async fn test_query_document_snapshot_id() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let collection_ref = firestore.collection(format!("rooms/{}/items", id))?;
    let document_reference = collection_ref.add(HashMap::<String, String>::new()).await?;
    let query_snapshot = collection_ref.get().await?;
    let docs = query_snapshot.docs();
    assert_eq!(docs.len(), 1);
    assert_eq!(docs[0].id(), document_reference.id());
    Ok(())
}
