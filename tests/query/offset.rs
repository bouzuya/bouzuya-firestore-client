// since v2.1 -> v3.0 (breaking change)
#[tokio::test]
async fn test_query_offset() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-query-offset")?;
    collection_reference
        .add(HashMap::<String, String>::new())
        .await?;
    collection_reference
        .add(HashMap::<String, String>::new())
        .await?;
    collection_reference
        .add(HashMap::<String, String>::new())
        .await?;
    let all = collection_reference.limit(1000)?.get().await?;
    let total = all.docs().len();
    let with_offset = collection_reference
        .limit(1000)?
        .offset(total as i32 - 1)?
        .get()
        .await?;
    assert_eq!(with_offset.docs().len(), 1);
    Ok(())
}

// since v3.0
#[tokio::test]
async fn test_query_offset_negative() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    let query = collection_reference.limit(1)?;
    assert!(query.offset(-1).is_err());
    Ok(())
}
