// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_document_snapshot_exists() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_reference = firestore.doc(format!("rooms/{}", id))?;

    // non-existing document
    let snapshot = document_reference.get().await?;
    assert!(!snapshot.exists());

    // existing document
    document_reference
        .create(HashMap::<String, String>::new())
        .await?;
    let snapshot = document_reference.get().await?;
    assert!(snapshot.exists());

    Ok(())
}
