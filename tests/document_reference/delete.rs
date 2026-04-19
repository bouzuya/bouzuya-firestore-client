// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_document_reference_delete() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Precondition;
    use bouzuya_firestore_client::WriteResult;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_ref = firestore.doc(format!("rooms/{}", id))?;

    assert!(!document_ref.get().await?.exists());

    // nonexistent document can be deleted
    let precondition = Precondition {
        exists: None,
        last_update_time: None,
    };
    let _: WriteResult = document_ref.delete(precondition).await?;
    assert!(!document_ref.get().await?.exists());

    let data: HashMap<String, String> = HashMap::new();
    document_ref.create(data).await?;
    assert!(document_ref.get().await?.exists());
    let precondition = Precondition {
        exists: None,
        last_update_time: None,
    };
    let _: WriteResult = document_ref.delete(precondition).await?;
    assert!(!document_ref.get().await?.exists());

    Ok(())
}
