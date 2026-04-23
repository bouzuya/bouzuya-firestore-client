// since v2.1
#[tokio::test]
async fn test_query_limit() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Query;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    collection_reference
        .add(std::collections::HashMap::<String, String>::new())
        .await?;
    collection_reference
        .add(std::collections::HashMap::<String, String>::new())
        .await?;
    collection_reference
        .add(std::collections::HashMap::<String, String>::new())
        .await?;
    let query: Query = collection_reference.offset(0);
    let query: Query = query.limit(2);
    assert_eq!(query.get().await?.size(), 2);
    Ok(())
}
