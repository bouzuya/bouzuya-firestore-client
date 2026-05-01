// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_firestore_get_all() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_reference = firestore.doc("rooms/test-get-all")?;
    let snapshots = firestore.get_all([document_reference]).await?;
    assert_eq!(snapshots.len(), 1);
    Ok(())
}

// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_firestore_get_all_multiple() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_reference1 = firestore.doc("rooms/room1")?;
    let document_reference2 = firestore.doc("rooms/room2")?;
    let document_reference3 = firestore.doc("rooms/room3")?;
    let document_snapshots = firestore.get_all([document_reference1, document_reference2, document_reference3]).await?;
    assert_eq!(document_snapshots.len(), 3);
    Ok(())
}

// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_firestore_get_all_returns_snapshot_even_if_not_exists() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_reference = firestore.doc("rooms/nonexistent-document")?;
    let snapshots = firestore.get_all([document_reference]).await?;
    assert_eq!(snapshots.len(), 1);
    assert!(!snapshots[0].exists());
    Ok(())
}

// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_firestore_get_all_returns_snapshot_with_exists_when_document_exists()
-> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Precondition;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_reference = firestore.doc(format!("rooms/{}", id))?;
    let data: HashMap<String, String> = HashMap::new();
    document_reference.create(data).await?;

    let snapshots = firestore.get_all([document_reference.clone()]).await?;
    assert_eq!(snapshots.len(), 1);
    assert!(snapshots[0].exists());

    document_reference
        .delete(Precondition {
            exists: None,
            last_update_time: None,
        })
        .await?;
    Ok(())
}
