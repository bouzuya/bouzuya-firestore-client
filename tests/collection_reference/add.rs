// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_collection_reference_add() -> anyhow::Result<()> {
    use bouzuya_firestore_client::DocumentReference;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("rooms")?;
    let document_reference: DocumentReference =
        collection_ref.add(HashMap::<String, String>::new()).await?;
    assert!(document_reference.path().starts_with("rooms/"));
    Ok(())
}
