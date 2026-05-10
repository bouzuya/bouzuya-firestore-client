// since v3.1
#[test]
fn test_collection_group_start_after() {
    fn _check<I>(
        collection_group: bouzuya_firestore_client::CollectionGroup,
        values: I,
    ) -> Result<bouzuya_firestore_client::Query, bouzuya_firestore_client::Error>
    where
        I: IntoIterator,
        I::Item: serde::Serialize,
    {
        collection_group.start_after(values)
    }
}

// since v3.1
#[tokio::test]
#[serial_test::serial]
async fn test_collection_group_start_after_get() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let sub_collection_name = format!("collection-group-start-after-{}", uuid::Uuid::now_v7());
    let document_reference1 = firestore
        .collection("projects")?
        .add(HashMap::<String, String>::new())
        .await?;
    for i in 1_i64..=3_i64 {
        document_reference1
            .collection(&sub_collection_name)?
            .add(
                [("n".to_owned(), i)]
                    .into_iter()
                    .collect::<HashMap<String, i64>>(),
            )
            .await?;
    }
    let document_reference2 = firestore
        .collection("rooms")?
        .add(HashMap::<String, String>::new())
        .await?;
    for i in 1_i64..=3_i64 {
        document_reference2
            .collection(&sub_collection_name)?
            .add(
                [("n".to_owned(), i)]
                    .into_iter()
                    .collect::<HashMap<String, i64>>(),
            )
            .await?;
    }
    let collection_group = firestore.collection_group(&sub_collection_name)?;
    let query_snapshot = collection_group
        .start_after(vec![2_i64])?
        .order_by("n", "asc")?
        .get()
        .await?;
    assert_eq!(query_snapshot.size(), 2);
    for query_document_snapshot in query_snapshot {
        let data = query_document_snapshot.data::<HashMap<String, i64>>()?;
        let n = data.get("n").copied();
        assert!(matches!(n, Some(n) if n > 2), "expected n > 2, got {:?}", n);
    }
    Ok(())
}

// since v3.1
#[tokio::test]
async fn test_collection_group_start_after_multiple_types() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    #[derive(serde::Serialize)]
    #[serde(untagged)]
    enum Mixed {
        I(i64),
        S(String),
    }
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_group = firestore.collection_group("messages")?;
    let _query = collection_group.start_after(vec![Mixed::S("Alice".to_string()), Mixed::I(30)])?;
    Ok(())
}
