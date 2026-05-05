// since v3.1
#[tokio::test]
async fn test_collection_group_limit() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Query;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    let document_reference = collection_reference
        .add(std::collections::HashMap::<String, String>::new())
        .await?;
    let sub_collection_reference = document_reference.collection("messages")?;
    sub_collection_reference
        .add(std::collections::HashMap::<String, String>::new())
        .await?;
    sub_collection_reference
        .add(std::collections::HashMap::<String, String>::new())
        .await?;
    sub_collection_reference
        .add(std::collections::HashMap::<String, String>::new())
        .await?;
    let collection_group = firestore.collection_group("messages")?;
    let query: Query = collection_group.limit(2)?;
    assert_eq!(query.get().await?.size(), 2);
    Ok(())
}

// since v3.1
#[tokio::test]
async fn test_collection_group_limit_negative() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_group = firestore.collection_group("messages")?;
    assert!(collection_group.limit(-1).is_err());
    Ok(())
}
