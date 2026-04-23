// since v2.1
#[tokio::test]
#[serial_test::serial]
async fn test_query_document_snapshot_ref() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::QueryDocumentSnapshot;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let collection_ref = firestore.collection(format!("rooms/{}/items", id))?;
    let document_ref = collection_ref.add(HashMap::<String, String>::new()).await?;
    let query_snapshot = collection_ref.get().await?;
    let docs = query_snapshot.docs();
    assert_eq!(docs.len(), 1);
    let query_document_snapshot: &QueryDocumentSnapshot = &docs[0];
    assert_eq!(query_document_snapshot.r#ref(), document_ref);
    Ok(())
}
