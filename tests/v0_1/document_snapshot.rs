#[tokio::test]
async fn test_document_snapshot_data() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::DocumentPath;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    use std::str::FromStr as _;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc(DocumentPath::from_str("rooms/roomA")?);
    let snapshot = document_ref.get().await?;
    let data: Option<Result<HashMap<String, String>, _>> = snapshot.data();
    assert!(data.is_none());

    // FIXME: add tests when the document exists and has data
    Ok(())
}

#[tokio::test]
async fn test_document_snapshot_exists() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::DocumentPath;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::str::FromStr as _;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc(DocumentPath::from_str("rooms/roomA")?);
    let snapshot = document_ref.get().await?;
    assert!(!snapshot.exists());

    // FIXME: add tests when the document exists
    Ok(())
}

#[tokio::test]
async fn test_document_snapshot_id() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::DocumentPath;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::str::FromStr as _;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc(DocumentPath::from_str("rooms/roomA")?);
    let snapshot = document_ref.get().await?;
    assert_eq!(snapshot.id().to_string(), "roomA");
    Ok(())
}

#[tokio::test]
async fn test_document_snapshot_ref() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::DocumentPath;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::str::FromStr as _;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc(DocumentPath::from_str("rooms/roomA")?);
    let snapshot = document_ref.get().await?;
    assert_eq!(snapshot.r#ref().path().to_string(), "rooms/roomA");
    Ok(())
}

#[test]
fn test_document_snapshot_import() {
    use bouzuya_firestore_client::DocumentSnapshot;
    let _: Option<DocumentSnapshot> = None;
}
