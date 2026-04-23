// since v1.1
#[tokio::test]
#[serial_test::serial]
async fn test_document_reference_list_collections() -> anyhow::Result<()> {
    use bouzuya_firestore_client::CollectionReference;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_reference = firestore.doc(format!("rooms/{}", id))?;
    let collection_refs: Vec<CollectionReference> = document_reference.list_collections().await?;
    assert!(collection_refs.is_empty());
    let sub_collection = document_reference.collection("messages")?;
    sub_collection.add(HashMap::<String, String>::new()).await?;
    let collection_refs: Vec<CollectionReference> = document_reference.list_collections().await?;
    assert!(!collection_refs.is_empty());
    for collection_reference in &collection_refs {
        assert_eq!(
            collection_reference.path(),
            format!("rooms/{}/messages", id)
        );
    }
    Ok(())
}
