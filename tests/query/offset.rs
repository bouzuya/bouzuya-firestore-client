// since v2.1
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
        .offset(total as i32 - 1)
        .get()
        .await?;
    assert_eq!(with_offset.docs().len(), 1);
    Ok(())
}
