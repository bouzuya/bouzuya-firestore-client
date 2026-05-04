// since v3.0
#[test]
fn test_query_start_at() {
    fn _check<I>(
        query: bouzuya_firestore_client::Query,
        values: I,
    ) -> Result<bouzuya_firestore_client::Query, bouzuya_firestore_client::Error>
    where
        I: IntoIterator,
        I::Item: serde::Serialize,
    {
        query.start_at(values)
    }
}

// since v3.0
#[tokio::test]
async fn test_query_start_at_empty() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    let query = collection_reference.limit(1)?;
    let result = query.start_at(Vec::<i32>::new());
    assert!(result.is_err());
    Ok(())
}

// since v3.0
#[tokio::test]
#[serial_test::serial]
async fn test_query_start_at_get() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-query-start-at")?;
    for i in 1_i64..=3_i64 {
        collection_reference
            .add(
                [("n".to_owned(), i)]
                    .into_iter()
                    .collect::<HashMap<String, i64>>(),
            )
            .await?;
    }
    let query = collection_reference
        .order_by("n", "asc")?
        .start_at(vec![2_i64])?;
    let query_snapshot = query.get().await?;
    assert!(!query_snapshot.empty());
    for query_document_snapshot in query_snapshot {
        let data = query_document_snapshot.data::<HashMap<String, i64>>()?;
        let n = data.get("n").copied();
        assert!(
            matches!(n, Some(n) if n >= 2),
            "expected n >= 2, got {:?}",
            n
        );
    }
    Ok(())
}

// since v3.0
#[tokio::test]
async fn test_query_start_at_multiple_types() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    #[derive(serde::Serialize)]
    #[serde(untagged)]
    enum Mixed {
        I(i64),
        S(String),
    }
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    let query = collection_reference.limit(1)?;
    let _query = query.start_at(vec![Mixed::S("Alice".to_string()), Mixed::I(30)])?;
    Ok(())
}
