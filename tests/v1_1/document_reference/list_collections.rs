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
    let document_ref = firestore.doc(format!("rooms/{}", id))?;
    let collection_refs: Vec<CollectionReference> = document_ref.list_collections().await?;
    assert!(collection_refs.is_empty());
    let sub_collection = document_ref.collection("messages")?;
    sub_collection.add(HashMap::<String, String>::new()).await?;
    let collection_refs: Vec<CollectionReference> = document_ref.list_collections().await?;
    assert!(!collection_refs.is_empty());
    for collection_ref in &collection_refs {
        assert_eq!(collection_ref.path(), format!("rooms/{}/messages", id));
    }
    Ok(())
}
