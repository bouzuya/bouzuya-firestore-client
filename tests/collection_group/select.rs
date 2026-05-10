// since v3.1
#[test]
fn test_collection_group_select() {
    fn _check<I>(
        collection_group: bouzuya_firestore_client::CollectionGroup,
        fields: I,
    ) -> Result<bouzuya_firestore_client::Query, bouzuya_firestore_client::Error>
    where
        I: IntoIterator,
        I::Item: bouzuya_firestore_client::IntoFieldPath,
    {
        collection_group.select(fields)
    }
}

// since v3.1
#[tokio::test]
#[serial_test::serial]
async fn test_collection_group_select_get() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    // CollectionReference::select のテストと同様に　テストデータを修正 (a, b のフィールドをもたせる)
    // arrange
    let sub_collection_name = format!("collection-group-select-{}", uuid::Uuid::now_v7());
    let document_reference1 = firestore
        .collection("projects")?
        .add(HashMap::<String, String>::new())
        .await?;
    for n in [3_i64, 1_i64, 2_i64] {
        document_reference1
            .collection(&sub_collection_name)?
            .add(
                [("a".to_owned(), n), ("b".to_owned(), n * 10)]
                    .into_iter()
                    .collect::<HashMap<String, i64>>(),
            )
            .await?;
    }
    let document_reference2 = firestore
        .collection("rooms")?
        .add(HashMap::<String, String>::new())
        .await?;
    for n in [3_i64, 1_i64, 2_i64] {
        document_reference2
            .collection(&sub_collection_name)?
            .add(
                [("a".to_owned(), n), ("b".to_owned(), n * 10)]
                    .into_iter()
                    .collect::<HashMap<String, i64>>(),
            )
            .await?;
    }
    // act
    let collection_group = firestore.collection_group(&sub_collection_name)?;
    let query_snapshot = collection_group.select(["a"])?.limit(10)?.get().await?;
    // assert
    assert!(!query_snapshot.empty());
    for query_document_snapshot in query_snapshot {
        let data = query_document_snapshot.data::<HashMap<String, i64>>()?;
        assert!(data.contains_key("a"));
        assert!(!data.contains_key("b"));
    }
    Ok(())
}
