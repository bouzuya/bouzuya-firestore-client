// since v2.1
#[tokio::test]
async fn test_collection_reference_offset() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("test-collection-reference-offset")?;
    collection_ref.add(HashMap::<String, String>::new()).await?;
    collection_ref.add(HashMap::<String, String>::new()).await?;
    collection_ref.add(HashMap::<String, String>::new()).await?;
    let all = collection_ref.limit(1000).get().await?;
    let total = all.docs().len();
    let with_offset = collection_ref
        .offset(total as i32 - 1)
        .limit(1000)
        .get()
        .await?;
    assert_eq!(with_offset.docs().len(), 1);
    Ok(())
}
