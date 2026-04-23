// since v0.1
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
    let document_reference = firestore.doc(format!("rooms/{}", id))?;
    let initial = HashMap::from([
        ("a".to_string(), "1".to_string()),
        ("b".to_string(), "2".to_string()),
    ]);
    document_reference.create(initial).await?;
    let new_data = HashMap::from([("a".to_string(), "updated".to_string())]);
    firestore
        .run_transaction(
            |transaction| {
                let document_reference = document_reference.clone();
                let new_data = new_data.clone();
                Box::pin(async move {
                    transaction.set(&document_reference, &new_data)?;
                    Ok(())
                })
            },
            TransactionOptions::default(),
        )
        .await?;
    let snapshot = document_reference.get().await?;
    let data: HashMap<String, String> = snapshot.data().ok_or(anyhow::anyhow!("no data"))??;
    assert_eq!(data.get("a").map(String::as_str), Some("updated"));
    assert_eq!(data.get("b"), None);
    Ok(())
}
