#[tokio::test]
async fn test_document_snapshot_create_time() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Timestamp;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_ref = firestore.doc(format!("rooms/{}", id))?;

    // non-existing document has no create_time
    let snapshot = document_ref.get().await?;
    assert_eq!(snapshot.create_time(), None);

    // existing document has create_time
    document_ref
        .create(HashMap::<String, String>::new())
        .await?;
    let snapshot = document_ref.get().await?;
    let create_time: Option<Timestamp> = snapshot.create_time();
    assert!(create_time.is_some_and(|t| t.to_millis() > 0));

    Ok(())
}

#[tokio::test]
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

#[tokio::test]
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

#[tokio::test]
async fn test_document_snapshot_id() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let snapshot = document_ref.get().await?;
    assert_eq!(snapshot.id().to_string(), "roomA");
    Ok(())
}

#[tokio::test]
async fn test_document_snapshot_update_time() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Timestamp;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_ref = firestore.doc(format!("rooms/{}", id))?;

    // non-existing document has no update_time
    let snapshot = document_ref.get().await?;
    assert_eq!(snapshot.update_time(), None);

    // existing document has update_time
    document_ref
        .create(HashMap::<String, String>::new())
        .await?;
    let snapshot = document_ref.get().await?;
    let update_time: Option<Timestamp> = snapshot.update_time();
    assert!(update_time.is_some_and(|t| t.to_millis() > 0));

    Ok(())
}

#[tokio::test]
async fn test_document_snapshot_ref() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let snapshot = document_ref.get().await?;
    assert_eq!(snapshot.r#ref().path().to_string(), "rooms/roomA");
    Ok(())
}

#[test]
fn test_document_snapshot_import() {
    use bouzuya_firestore_client::DocumentSnapshot;
    let _: Option<DocumentSnapshot> = None;
}
