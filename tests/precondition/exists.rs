// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_precondition_exists() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Precondition;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_ref = firestore.doc(format!("rooms/{}", id))?;

    // exists: Some(true) on non-existing document should fail
    let precondition = Precondition {
        exists: Some(true),
        last_update_time: None,
    };
    assert!(document_ref.delete(precondition).await.is_err());

    // create the document
    document_ref
        .create(HashMap::<String, String>::new())
        .await?;

    // exists: Some(true) on existing document should succeed
    let precondition = Precondition {
        exists: Some(true),
        last_update_time: None,
    };
    assert!(document_ref.delete(precondition).await.is_ok());

    Ok(())
}
