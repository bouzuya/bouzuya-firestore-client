// since v2.1
#[tokio::test]
#[serial_test::serial]
async fn test_query_document_snapshot_clone() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::QueryDocumentSnapshot;
    use std::collections::HashMap;

    fn assert_impl<T: Clone>() {}
    assert_impl::<QueryDocumentSnapshot>();

    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let collection_reference = firestore.collection(format!("rooms/{}/items", id))?;
    collection_reference
        .add(HashMap::<String, String>::new())
        .await?;
    let query_snapshot = collection_reference.get().await?;
    let query_document_snapshots = query_snapshot.docs();
    assert_eq!(query_document_snapshots.len(), 1);
    let cloned = query_document_snapshots[0].clone();
    assert_eq!(cloned.id(), query_document_snapshots[0].id());
    Ok(())
}
