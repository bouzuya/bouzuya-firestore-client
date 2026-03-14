#[tokio::test]
#[serial_test::serial]
async fn test_document_snapshot_id() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let snapshot = document_ref.get().await?;
    assert_eq!(snapshot.id().to_string(), "roomA");
    Ok(())
}
