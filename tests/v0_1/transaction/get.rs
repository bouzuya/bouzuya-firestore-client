#[tokio::test]
#[serial_test::serial]
async fn test_transaction_get() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::TransactionOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_ref = firestore.doc(format!("rooms/{}", id))?;
    let initial = HashMap::from([("a".to_string(), "1".to_string())]);
    document_ref.create(initial).await?;
    let snapshot = firestore
        .run_transaction(
            |transaction| {
                let document_ref = document_ref.clone();
                Box::pin(async move { transaction.get(&document_ref).await })
            },
            TransactionOptions::default(),
        )
        .await?;
    assert!(snapshot.exists());
    let data: HashMap<String, String> = snapshot.data().ok_or(anyhow::anyhow!("no data"))??;
    assert_eq!(data.get("a").map(String::as_str), Some("1"));
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_transaction_get_not_found() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::TransactionOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_ref = firestore.doc(format!("rooms/{}", id))?;
    let snapshot = firestore
        .run_transaction(
            |transaction| {
                let document_ref = document_ref.clone();
                Box::pin(async move { transaction.get(&document_ref).await })
            },
            TransactionOptions::default(),
        )
        .await?;
    assert!(!snapshot.exists());
    Ok(())
}
