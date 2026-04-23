// since v2.1
#[tokio::test]
#[serial_test::serial]
async fn test_query_snapshot_docs() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    collection_reference
        .add(HashMap::<String, String>::new())
        .await?;
    let query_snapshot = collection_reference.get().await?;
    let query_document_snapshots = query_snapshot.docs();
    assert!(!query_document_snapshots.is_empty());
    for query_document_snapshot in query_document_snapshots {
        assert!(query_document_snapshot.exists());
    }
    Ok(())
}
