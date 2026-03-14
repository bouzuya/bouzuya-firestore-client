#[tokio::test]
#[serial_test::serial]
async fn test_document_snapshot_ref() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let snapshot = document_ref.get().await?;
    assert_eq!(snapshot.r#ref().path().to_string(), "rooms/roomA");
    Ok(())
}
