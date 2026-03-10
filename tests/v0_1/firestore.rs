#[test]
fn test_firestore_import() {
    use bouzuya_firestore_client::Firestore;
    let _: Option<Firestore> = None;
}

#[tokio::test]
async fn test_firestore_new() {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let options = FirestoreOptions::default();
    assert!(Firestore::new(options).is_ok());
}

#[tokio::test]
async fn test_firestore_collection() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("rooms")?;
    assert_eq!(collection_ref.id().to_string(), "rooms");
    Ok(())
}

#[tokio::test]
async fn test_firestore_doc() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    assert_eq!(document_ref.id().to_string(), "roomA");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_firestore_run_transaction() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Error;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::TransactionOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let options = TransactionOptions {
        max_attempts: None,
        read_only: None,
        read_time: None,
    };
    let result = firestore
        .run_transaction(
            |_transaction| Box::pin(async { Ok::<(), Error>(()) }),
            options,
        )
        .await;
    assert!(result.is_ok());
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_firestore_run_transaction_return_value() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Error;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::TransactionOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let options = TransactionOptions {
        max_attempts: None,
        read_only: None,
        read_time: None,
    };
    let value = firestore
        .run_transaction(
            |_transaction| Box::pin(async { Ok::<i32, Error>(42) }),
            options,
        )
        .await?;
    assert_eq!(value, 42);
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_firestore_run_transaction_error() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Error;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::TransactionOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let options = TransactionOptions {
        max_attempts: None,
        read_only: None,
        read_time: None,
    };
    // Obtain an Error by triggering an invalid document path parse
    let err = firestore
        .doc("rooms")
        .err()
        .ok_or_else(|| anyhow::anyhow!("expected error for invalid doc path"))?;
    let result = firestore
        .run_transaction(
            move |_transaction| Box::pin(async move { Err::<(), Error>(err) }),
            options,
        )
        .await;
    assert!(result.is_err());
    Ok(())
}
