// since v0.1
#[test]
fn test_firestore_options_database_id() {
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore_options = FirestoreOptions {
        database_id: Some("my-database".to_owned()),
        project_id: None,
    };
    assert_eq!(
        firestore_options.database_id,
        Some("my-database".to_owned())
    );
}
