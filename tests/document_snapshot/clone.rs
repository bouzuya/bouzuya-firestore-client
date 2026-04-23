// since v2.1
#[tokio::test]
#[serial_test::serial]
async fn test_document_snapshot_clone() -> anyhow::Result<()> {
    use bouzuya_firestore_client::DocumentSnapshot;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;

    fn assert_impl<T: Clone>() {}
    assert_impl::<DocumentSnapshot>();

    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_reference = firestore.doc(format!("rooms/{}", id))?;
    document_reference
        .create(HashMap::<String, String>::new())
        .await?;
    let snapshot = document_reference.get().await?;
    let cloned = snapshot.clone();
    assert_eq!(snapshot.id(), cloned.id());
    assert_eq!(snapshot.exists(), cloned.exists());
    assert_eq!(snapshot.create_time(), cloned.create_time());
    Ok(())
}
