#[test]
fn test_collection_reference_import() {
    use bouzuya_firestore_client::CollectionReference;
    let _: Option<CollectionReference> = None;
}

#[test]
fn test_collection_reference_id() {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default()).unwrap();
    let collection_ref = firestore.collection("collection");
    assert_eq!(collection_ref.id(), "collection");
}
