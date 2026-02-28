#[test]
fn test_firestore_options_import() {
    use bouzuya_firestore_client::FirestoreOptions;
    let _: Option<FirestoreOptions> = None;
}

#[test]
fn test_firestore_options_default() {
    use bouzuya_firestore_client::FirestoreOptions;

    fn assert_default<T: Default>() {}

    assert_default::<FirestoreOptions>();
    let _: FirestoreOptions = FirestoreOptions::default();
}
