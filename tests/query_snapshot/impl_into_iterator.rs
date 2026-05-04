// since v3.0
#[tokio::test]
#[serial_test::serial]
async fn test_query_snapshot_into_iter() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    collection_reference
        .add(HashMap::<String, String>::new())
        .await?;
    let query_snapshot = collection_reference.get().await?;
    for query_document_snapshot in query_snapshot {
        assert!(query_document_snapshot.exists());
    }
    Ok(())
}
