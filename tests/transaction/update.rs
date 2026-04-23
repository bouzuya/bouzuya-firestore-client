// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_transaction_update() -> anyhow::Result<()> {
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
    let document_reference = firestore.doc(format!("rooms/{}", id))?;
    let initial = HashMap::from([
        ("a".to_string(), "1".to_string()),
        ("b".to_string(), "2".to_string()),
    ]);
    document_reference.create(initial).await?;
    let update_data = HashMap::from([("a".to_string(), "updated".to_string())]);
    firestore
        .run_transaction(
            |transaction| {
                let document_reference = document_reference.clone();
                let update_data = update_data.clone();
                Box::pin(async move {
                    transaction.update(&document_reference, &update_data, Precondition::default())?;
                    Ok(())
                })
            },
            TransactionOptions::default(),
        )
        .await?;
    let snapshot = document_reference.get().await?;
    let data: HashMap<String, String> = snapshot.data().ok_or(anyhow::anyhow!("no data"))??;
    assert_eq!(data.get("a").map(String::as_str), Some("updated"));
    assert_eq!(data.get("b").map(String::as_str), Some("2"));
    Ok(())
}

// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_transaction_update_with_precondition() -> anyhow::Result<()> {
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
    let document_reference = firestore.doc(format!("rooms/{}", id))?;
    let initial = HashMap::from([("a".to_string(), "1".to_string())]);
    document_reference.create(initial).await?;
    let update_data = HashMap::from([("a".to_string(), "updated".to_string())]);
    firestore
        .run_transaction(
            |transaction| {
                let document_reference = document_reference.clone();
                let update_data = update_data.clone();
                Box::pin(async move {
                    transaction.update(
                        &document_reference,
                        &update_data,
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
    let snapshot = document_reference.get().await?;
    let data: HashMap<String, String> = snapshot.data().ok_or(anyhow::anyhow!("no data"))??;
    assert_eq!(data.get("a").map(String::as_str), Some("updated"));
    Ok(())
}
