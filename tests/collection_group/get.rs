// since v3.1
#[tokio::test]
#[serial_test::serial]
async fn test_collection_group_get() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::QuerySnapshot;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_group = firestore.collection_group("messages")?;
    let collection_reference = firestore.collection("rooms")?;
    let document_reference = collection_reference
        .add(HashMap::<String, String>::new())
        .await?;
    let sub_collection_reference = document_reference.collection("messages")?;
    sub_collection_reference
        .add(HashMap::<String, String>::new())
        .await?;
    let query_snapshot: QuerySnapshot = collection_group.get().await?;
    assert!(!query_snapshot.empty());
    for query_document_snapshot in query_snapshot {
        assert!(query_document_snapshot.exists());
    }
    Ok(())
}
