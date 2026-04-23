// since v1.1
#[tokio::test]
#[serial_test::serial]
async fn test_document_reference_set() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::WriteResult;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_reference = firestore.doc(format!("rooms/{}", id))?;
    let data = HashMap::from([("a".to_string(), "1".to_string())]);
    let _: WriteResult = document_reference.set(data).await?;
    let snapshot = document_reference.get().await?;
    let got: HashMap<String, String> = snapshot.data().ok_or(anyhow::anyhow!("no data"))??;
    assert_eq!(got.get("a").map(String::as_str), Some("1"));
    Ok(())
}

// since v1.1
#[tokio::test]
#[serial_test::serial]
async fn test_document_reference_set_overwrites() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::WriteResult;
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
    let _: WriteResult = document_reference.set(new_data).await?;
    let snapshot = document_reference.get().await?;
    let got: HashMap<String, String> = snapshot.data().ok_or(anyhow::anyhow!("no data"))??;
    assert_eq!(got.get("a").map(String::as_str), Some("updated"));
    // b should be removed because set overwrites the entire document
    assert_eq!(got.get("b"), None);
    Ok(())
}
