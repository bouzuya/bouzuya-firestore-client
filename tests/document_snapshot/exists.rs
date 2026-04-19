// since v0.1
#[tokio::test]
#[serial_test::serial]
async fn test_document_snapshot_exists() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let snapshot = document_ref.get().await?;
    assert!(!snapshot.exists());

    // FIXME: add tests when the document exists
    Ok(())
}
