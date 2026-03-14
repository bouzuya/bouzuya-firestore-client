#[tokio::test]
#[serial_test::serial]
async fn test_precondition_last_update_time() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Precondition;
    use bouzuya_firestore_client::Timestamp;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_ref = firestore.doc(format!("rooms/{}", id))?;
    let write_result = document_ref
        .create(HashMap::<String, String>::new())
        .await?;
    let write_time = write_result.write_time();

    // wrong last_update_time should fail
    let precondition = Precondition {
        exists: None,
        last_update_time: Some(Timestamp::from_millis(0)),
    };
    assert!(document_ref.delete(precondition).await.is_err());

    // correct last_update_time should succeed
    let precondition = Precondition {
        exists: None,
        last_update_time: Some(write_time),
    };
    assert!(document_ref.delete(precondition).await.is_ok());

    Ok(())
}
