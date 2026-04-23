// since v2.1
#[tokio::test]
#[serial_test::serial]
async fn test_query_get() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::QuerySnapshot;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-query-get")?;
    collection_reference
        .add(
            [("k".to_owned(), "v".to_owned())]
                .into_iter()
                .collect::<std::collections::HashMap<String, String>>(),
        )
        .await?;
    collection_reference
        .add(
            [("k".to_owned(), "v".to_owned())]
                .into_iter()
                .collect::<std::collections::HashMap<String, String>>(),
        )
        .await?;
    let query = collection_reference.limit(1);
    let query_snapshot: QuerySnapshot = query.get().await?;
    assert!(query_snapshot.docs().len() <= 1);
    Ok(())
}
