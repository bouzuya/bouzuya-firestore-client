// since v2.1 -> v3.0 (breaking change)
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
    let query: Query = collection_reference.limit(2)?;
    assert_eq!(query.get().await?.size(), 2);
    Ok(())
}

// since v3.0
#[tokio::test]
async fn test_collection_reference_limit_negative() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    assert!(collection_reference.limit(-1).is_err());
    Ok(())
}
