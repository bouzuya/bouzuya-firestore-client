#[tokio::test]
#[serial_test::serial]
async fn test_document_snapshot_data() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let snapshot = document_ref.get().await?;
    let data: Option<Result<HashMap<String, String>, _>> = snapshot.data();
    assert!(data.is_none());

    // FIXME: add tests when the document exists and has data
    Ok(())
}
