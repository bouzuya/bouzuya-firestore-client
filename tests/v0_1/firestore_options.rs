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
    let firestore_options = FirestoreOptions::default();
    assert_eq!(firestore_options.project_id, None);
}

#[test]
fn test_firestore_options_project_id() {
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore_options = FirestoreOptions {
        project_id: Some("my-project".to_owned()),
    };
    assert_eq!(firestore_options.project_id, Some("my-project".to_owned()));
}
