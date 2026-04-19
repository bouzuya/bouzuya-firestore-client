// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_collection_reference_list_documents() -> anyhow::Result<()> {
    use bouzuya_firestore_client::DocumentReference;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("rooms")?;
    collection_ref.add(HashMap::<String, String>::new()).await?;
    let document_refs: Vec<DocumentReference> = collection_ref.list_documents().await?;
    assert!(!document_refs.is_empty());
    for document_ref in &document_refs {
        assert!(document_ref.path().starts_with("rooms/"));
    }
    Ok(())
}
