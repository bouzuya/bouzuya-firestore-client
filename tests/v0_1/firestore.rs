#[test]
fn test_firestore_import() {
    use bouzuya_firestore_client::Firestore;
    let _: Option<Firestore> = None;
}

#[test]
fn test_firestore_new() {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let options = FirestoreOptions::default();
    assert!(Firestore::new(options).is_ok());
}

#[test]
fn test_firestore_collection() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::CollectionId;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::str::FromStr as _;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_id = CollectionId::from_str("rooms")?;
    let collection_ref = firestore.collection(collection_id);
    assert_eq!(collection_ref.id().to_string(), "rooms");
    Ok(())
}
