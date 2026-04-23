// since v2.1
#[tokio::test]
async fn test_collection_reference_limit() -> anyhow::Result<()> {
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
    let query: Query = collection_reference.limit(2);
    assert_eq!(query.get().await?.size(), 2);
    Ok(())
}
