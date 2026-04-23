// since v2.1
#[tokio::test]
#[serial_test::serial]
async fn test_query_snapshot_empty() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Precondition;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("query_snapshot_empty")?;
    for document_reference in collection_reference.list_documents().await? {
        document_reference.delete(Precondition::default()).await?;
    }
    let query_snapshot = collection_reference.get().await?;
    assert!(query_snapshot.empty());
    collection_reference
        .add(HashMap::<String, String>::new())
        .await?;
    let query_snapshot = collection_reference.get().await?;
    assert!(!query_snapshot.empty());
    Ok(())
}
