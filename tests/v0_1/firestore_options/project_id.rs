#[test]
fn test_firestore_options_project_id() {
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore_options = FirestoreOptions {
        project_id: Some("my-project".to_owned()),
    };
    assert_eq!(firestore_options.project_id, Some("my-project".to_owned()));
}
