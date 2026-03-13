#[tokio::test]
#[serial_test::serial]
async fn test_transaction_create() -> anyhow::Result<()> {
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
    let data = HashMap::<String, String>::new();
    firestore
        .run_transaction(
            |transaction| {
                let document_ref = document_ref.clone();
                Box::pin(async move {
                    transaction.create(&document_ref, &data)?;
                    Ok(())
                })
            },
            TransactionOptions::default(),
        )
        .await?;
    assert!(document_ref.get().await?.exists());
    Ok(())
}

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

#[tokio::test]
#[serial_test::serial]
async fn test_transaction_set() -> anyhow::Result<()> {
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
    let initial = HashMap::from([
        ("a".to_string(), "1".to_string()),
        ("b".to_string(), "2".to_string()),
    ]);
    document_ref.create(initial).await?;
    let new_data = HashMap::from([("a".to_string(), "updated".to_string())]);
    firestore
        .run_transaction(
            |transaction| {
                let document_ref = document_ref.clone();
                let new_data = new_data.clone();
                Box::pin(async move {
                    transaction.set(&document_ref, &new_data)?;
                    Ok(())
                })
            },
            TransactionOptions::default(),
        )
        .await?;
    let snapshot = document_ref.get().await?;
    let data: HashMap<String, String> = snapshot.data().ok_or(anyhow::anyhow!("no data"))??;
    assert_eq!(data.get("a").map(String::as_str), Some("updated"));
    assert_eq!(data.get("b"), None);
    Ok(())
}

#[test]
fn test_transaction_import() {
    use bouzuya_firestore_client::Transaction;
    let _: Option<Transaction> = None;
}
