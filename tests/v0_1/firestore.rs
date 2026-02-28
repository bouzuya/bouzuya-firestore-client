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
