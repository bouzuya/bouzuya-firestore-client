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

#[test]
fn test_transaction_import() {
    use bouzuya_firestore_client::Transaction;
    let _: Option<Transaction> = None;
}
