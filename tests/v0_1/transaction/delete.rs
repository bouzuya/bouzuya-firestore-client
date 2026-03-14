#[tokio::test]
#[serial_test::serial]
async fn test_transaction_delete() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Precondition;
    use bouzuya_firestore_client::TransactionOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_ref = firestore.doc(format!("rooms/{}", id))?;
    let data = HashMap::<String, String>::new();
    document_ref.create(data).await?;
    assert!(document_ref.get().await?.exists());
    firestore
        .run_transaction(
            |transaction| {
                let document_ref = document_ref.clone();
                Box::pin(async move {
                    transaction.delete(&document_ref, Precondition::default())?;
                    Ok(())
                })
            },
            TransactionOptions::default(),
        )
        .await?;
    assert!(!document_ref.get().await?.exists());
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_transaction_delete_with_precondition() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Precondition;
    use bouzuya_firestore_client::TransactionOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_ref = firestore.doc(format!("rooms/{}", id))?;
    let data = HashMap::<String, String>::new();
    document_ref.create(data).await?;
    assert!(document_ref.get().await?.exists());
    firestore
        .run_transaction(
            |transaction| {
                let document_ref = document_ref.clone();
                Box::pin(async move {
                    transaction.delete(
                        &document_ref,
                        Precondition {
                            exists: Some(true),
                            last_update_time: None,
                        },
                    )?;
                    Ok(())
                })
            },
            TransactionOptions::default(),
        )
        .await?;
    assert!(!document_ref.get().await?.exists());
    Ok(())
}
