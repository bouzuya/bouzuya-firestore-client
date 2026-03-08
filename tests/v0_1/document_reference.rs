#[tokio::test]
async fn test_document_reference_clone() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::DocumentReference;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let cloned = document_ref.clone();
    assert_eq!(cloned.id().to_string(), "roomA");

    fn assert_fn<T: Clone>() {}
    assert_fn::<DocumentReference>();
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_document_reference_create() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::WriteResult;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_ref = firestore.doc(format!("rooms/{}", id))?;
    let data: HashMap<String, String> = HashMap::new();
    let _: WriteResult = document_ref.create(data).await?;
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_document_reference_delete() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Precondition;
    use bouzuya_firestore_client::WriteResult;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos()
        .to_string();
    let document_ref = firestore.doc(format!("rooms/{}", id))?;

    assert!(!document_ref.get().await?.exists());

    // nonexistent document can be deleted
    let precondition = Precondition {
        exists: None,
        last_update_time: None,
    };
    let _: WriteResult = document_ref.delete(precondition).await?;
    assert!(!document_ref.get().await?.exists());

    let data: HashMap<String, String> = HashMap::new();
    document_ref.create(data).await?;
    assert!(document_ref.get().await?.exists());
    let precondition = Precondition {
        exists: None,
        last_update_time: None,
    };
    let _: WriteResult = document_ref.delete(precondition).await?;
    assert!(!document_ref.get().await?.exists());

    Ok(())
}

#[tokio::test]
async fn test_document_reference_get() -> Result<(), bouzuya_firestore_client::Error> {
    // FIXME: add tests when the document exists and does not exist
    Ok(())
}

#[test]
fn test_document_reference_import() {
    use bouzuya_firestore_client::DocumentReference;
    let _: Option<DocumentReference> = None;
}

#[tokio::test]
async fn test_document_reference_collection() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let collection_ref = document_ref.collection("messages")?;
    assert_eq!(collection_ref.path().to_string(), "rooms/roomA/messages");
    Ok(())
}

#[tokio::test]
async fn test_document_reference_id() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let document_id = document_ref.id();
    assert_eq!(document_id.to_string(), "roomA");
    Ok(())
}

#[tokio::test]
async fn test_document_reference_parent() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let parent = document_ref.parent();
    assert_eq!(parent.path().to_string(), "rooms");
    let document_ref = firestore.doc("rooms/roomA/messages/message1")?;
    let parent = document_ref.parent();
    assert_eq!(parent.path().to_string(), "rooms/roomA/messages");
    Ok(())
}

#[tokio::test]
async fn test_document_reference_path() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let path = document_ref.path();
    assert_eq!(path.to_string(), "rooms/roomA");
    let document_ref = firestore.doc("rooms/roomA/messages/message1")?;
    let path = document_ref.path();
    assert_eq!(path.to_string(), "rooms/roomA/messages/message1");
    Ok(())
}
