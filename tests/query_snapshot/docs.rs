// since v2.1
#[tokio::test]
#[serial_test::serial]
async fn test_query_snapshot_docs() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("rooms")?;
    collection_ref.add(HashMap::<String, String>::new()).await?;
    let query_snapshot = collection_ref.get().await?;
    let docs = query_snapshot.docs();
    assert!(!docs.is_empty());
    for doc in docs {
        assert!(doc.exists());
    }
    Ok(())
}
