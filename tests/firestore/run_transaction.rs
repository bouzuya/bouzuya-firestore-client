// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_firestore_run_transaction() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Error;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::TransactionOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let result = firestore
        .run_transaction(
            |_transaction| Box::pin(async { Ok::<(), Error>(()) }),
            TransactionOptions::default(),
        )
        .await;
    assert!(result.is_ok());
    Ok(())
}

// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_firestore_run_transaction_return_value() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Error;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::TransactionOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let value = firestore
        .run_transaction(
            |_transaction| Box::pin(async { Ok::<i32, Error>(42) }),
            TransactionOptions::default(),
        )
        .await?;
    assert_eq!(value, 42);
    Ok(())
}

// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_firestore_run_transaction_error() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Error;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::TransactionOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    // Obtain an Error by triggering an invalid document path parse
    let err = firestore
        .doc("rooms")
        .err()
        .ok_or_else(|| anyhow::anyhow!("expected error for invalid doc path"))?;
    let result = firestore
        .run_transaction(
            move |_transaction| Box::pin(async move { Err::<(), Error>(err) }),
            TransactionOptions::default(),
        )
        .await;
    assert!(result.is_err());
    Ok(())
}
