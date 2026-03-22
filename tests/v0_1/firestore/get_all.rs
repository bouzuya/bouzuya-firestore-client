#[tokio::test]
#[serial_test::serial]
async fn test_firestore_get_all() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let doc_ref = firestore.doc("rooms/test-get-all")?;
    let snapshots = firestore.get_all([doc_ref]).await?;
    assert_eq!(snapshots.len(), 1);
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_firestore_get_all_multiple() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let doc_ref1 = firestore.doc("rooms/room1")?;
    let doc_ref2 = firestore.doc("rooms/room2")?;
    let doc_ref3 = firestore.doc("rooms/room3")?;
    let snapshots = firestore.get_all([doc_ref1, doc_ref2, doc_ref3]).await?;
    assert_eq!(snapshots.len(), 3);
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_firestore_get_all_returns_snapshot_even_if_not_exists() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let doc_ref = firestore.doc("rooms/nonexistent-document")?;
    let snapshots = firestore.get_all([doc_ref]).await?;
    assert_eq!(snapshots.len(), 1);
    assert!(!snapshots[0].exists());
    Ok(())
}
