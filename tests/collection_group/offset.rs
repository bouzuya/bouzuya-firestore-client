// since v3.1
#[tokio::test]
async fn test_collection_group_offset() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-collection-group-offset")?;
    let document_reference = collection_reference
        .add(HashMap::<String, String>::new())
        .await?;
    let sub_collection_reference = document_reference.collection("sub-collection")?;
    sub_collection_reference
        .add(HashMap::<String, String>::new())
        .await?;
    sub_collection_reference
        .add(HashMap::<String, String>::new())
        .await?;
    sub_collection_reference
        .add(HashMap::<String, String>::new())
        .await?;
    let all = firestore
        .collection_group("sub-collection")?
        .limit(100)?
        .get()
        .await?;
    let total = all.docs().len();
    let with_offset = firestore
        .collection_group("sub-collection")?
        .offset(total as i32 - 1)?
        .limit(1000)?
        .get()
        .await?;
    assert_eq!(with_offset.docs().len(), 1);
    Ok(())
}

// since v3.1
#[tokio::test]
async fn test_collection_group_offset_negative() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_group = firestore.collection_group("messages")?;
    assert!(collection_group.offset(-1).is_err());
    Ok(())
}
