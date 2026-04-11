#[tokio::test]
#[serial_test::serial]
async fn test_document_reference_update_with_precondition_exists_error() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Precondition;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_ref = firestore.doc(format!("rooms/{}", id))?;
    // document does not exist
    let update_data = std::collections::HashMap::from([("a".to_string(), "updated".to_string())]);
    let result = document_ref
        .update(
            update_data,
            Precondition {
                exists: Some(true),
                last_update_time: None,
            },
        )
        .await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_document_reference_update_with_precondition_last_update_time_error()
-> anyhow::Result<()> {
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
    let initial = HashMap::from([("a".to_string(), "1".to_string())]);
    document_ref.create(initial).await?;
    let update_data = HashMap::from([("a".to_string(), "updated".to_string())]);
    // use a mismatched last_update_time
    let result = document_ref
        .update(
            update_data,
            Precondition {
                exists: None,
                last_update_time: Some(Timestamp::from_millis(0)),
            },
        )
        .await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_document_reference_update_with_precondition_exists() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Precondition;
    use bouzuya_firestore_client::WriteResult;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_ref = firestore.doc(format!("rooms/{}", id))?;
    let initial = HashMap::from([("a".to_string(), "1".to_string())]);
    document_ref.create(initial).await?;
    let update_data = HashMap::from([("a".to_string(), "updated".to_string())]);
    let _: WriteResult = document_ref
        .update(
            update_data,
            Precondition {
                exists: Some(true),
                last_update_time: None,
            },
        )
        .await?;
    let snapshot = document_ref.get().await?;
    let data: HashMap<String, String> = snapshot.data().ok_or(anyhow::anyhow!("no data"))??;
    assert_eq!(data.get("a").map(String::as_str), Some("updated"));
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_document_reference_update_with_precondition_last_update_time() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Precondition;
    use bouzuya_firestore_client::WriteResult;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_ref = firestore.doc(format!("rooms/{}", id))?;
    let initial = HashMap::from([("a".to_string(), "1".to_string())]);
    let write_result = document_ref.create(initial).await?;
    let update_data = HashMap::from([("a".to_string(), "updated".to_string())]);
    let _: WriteResult = document_ref
        .update(
            update_data,
            Precondition {
                exists: None,
                last_update_time: Some(write_result.write_time()),
            },
        )
        .await?;
    let snapshot = document_ref.get().await?;
    let data: HashMap<String, String> = snapshot.data().ok_or(anyhow::anyhow!("no data"))??;
    assert_eq!(data.get("a").map(String::as_str), Some("updated"));
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_document_reference_update() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Precondition;
    use bouzuya_firestore_client::WriteResult;
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
    let update_data = HashMap::from([("a".to_string(), "updated".to_string())]);
    let _: WriteResult = document_ref
        .update(update_data, Precondition::default())
        .await?;
    let snapshot = document_ref.get().await?;
    let data: HashMap<String, String> = snapshot.data().ok_or(anyhow::anyhow!("no data"))??;
    assert_eq!(data.get("a").map(String::as_str), Some("updated"));
    assert_eq!(data.get("b").map(String::as_str), Some("2"));
    Ok(())
}
