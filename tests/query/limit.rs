// since v2.1 -> v3.0 (breaking change)
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
    let query: Query = query.limit(2)?;
    assert_eq!(query.get().await?.size(), 2);
    Ok(())
}

// since v3.0
#[tokio::test]
async fn test_query_limit_negative() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    let query = collection_reference.offset(0);
    assert!(query.limit(-1).is_err());
    Ok(())
}
