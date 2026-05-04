// since v3.0
#[tokio::test]
async fn test_firestore_database_id() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions {
        database_id: Some("my-database".to_owned()),
        project_id: Some("demo-project".to_owned()),
    })?;
    assert_eq!(firestore.database_id(), "my-database");
    Ok(())
}
