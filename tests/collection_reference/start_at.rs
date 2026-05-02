// since v2.2
#[test]
fn test_collection_reference_start_at() {
    fn _check<I>(
        collection_reference: bouzuya_firestore_client::CollectionReference,
        values: I,
    ) -> Result<bouzuya_firestore_client::Query, bouzuya_firestore_client::Error>
    where
        I: IntoIterator,
        I::Item: serde::Serialize,
    {
        collection_reference.start_at(values)
    }
}

// since v2.2
#[tokio::test]
#[serial_test::serial]
async fn test_collection_reference_start_at_get() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-collection-reference-start-at")?;
    for i in 1_i64..=3_i64 {
        collection_reference
            .add(
                [("n".to_owned(), i)]
                    .into_iter()
                    .collect::<HashMap<String, i64>>(),
            )
            .await?;
    }
    let query_snapshot = collection_reference
        .start_at(vec![2_i64])?
        .order_by("n", "asc")?
        .get()
        .await?;
    assert!(!query_snapshot.docs().is_empty());
    for query_document_snapshot in query_snapshot.docs() {
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

// since v2.2
#[tokio::test]
async fn test_collection_reference_start_at_multiple_types() -> anyhow::Result<()> {
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
    let _query =
        collection_reference.start_at(vec![Mixed::S("Alice".to_string()), Mixed::I(30)])?;
    Ok(())
}
