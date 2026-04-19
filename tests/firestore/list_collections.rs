// since v1.1
#[tokio::test]
#[serial_test::serial]
async fn test_firestore_list_collections() -> anyhow::Result<()> {
    use bouzuya_firestore_client::CollectionReference;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let collection_name = format!("test-list-collections-{}", id);
    let collection = firestore.collection(collection_name.clone())?;
    let collection_refs: Vec<CollectionReference> = firestore.list_collections().await?;
    assert!(!collection_refs.iter().any(|c| c.id() == collection_name));
    collection.add(HashMap::<String, String>::new()).await?;
    let collection_refs: Vec<CollectionReference> = firestore.list_collections().await?;
    assert!(collection_refs.iter().any(|c| c.id() == collection_name));
    Ok(())
}
