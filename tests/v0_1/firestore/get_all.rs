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
