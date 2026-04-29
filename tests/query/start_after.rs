// since v2.2
#[test]
fn test_query_start_after() {
    fn _check<I>(
        query: bouzuya_firestore_client::Query,
        values: I,
    ) -> Result<bouzuya_firestore_client::Query, bouzuya_firestore_client::Error>
    where
        I: IntoIterator,
        I::Item: serde::Serialize,
    {
        query.start_after(values)
    }
}

// since v2.2
#[tokio::test]
async fn test_query_start_after_empty() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    let query = collection_reference.limit(1);
    let result = query.start_after(Vec::<i32>::new());
    assert!(result.is_err());
    Ok(())
}

// since v2.2
#[tokio::test]
async fn test_query_start_after_multiple_types() -> anyhow::Result<()> {
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
    let query = collection_reference.limit(1);
    let _query = query.start_after(vec![Mixed::S("Alice".to_string()), Mixed::I(30)])?;
    Ok(())
}
