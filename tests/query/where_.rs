// since v2.2
#[test]
fn test_query_where_() {
    fn _check(
        query: bouzuya_firestore_client::Query,
        filter: bouzuya_firestore_client::Filter,
    ) -> bouzuya_firestore_client::Query {
        query.r#where(filter)
    }
}

// since v2.2
#[tokio::test]
#[serial_test::serial]
async fn test_query_where_get() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Filter;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-query-where")?;
    collection_reference
        .add(
            [("k".to_owned(), "target".to_owned())]
                .into_iter()
                .collect::<HashMap<String, String>>(),
        )
        .await?;
    collection_reference
        .add(
            [("k".to_owned(), "other".to_owned())]
                .into_iter()
                .collect::<HashMap<String, String>>(),
        )
        .await?;
    let filter = Filter::r#where("k".to_string(), "==", "target".to_string())?;
    // FIXME: Add CollectionReference::r#where() and remove this workaround
    let query = collection_reference.offset(0).r#where(filter);
    let query_snapshot = query.get().await?;
    assert!(!query_snapshot.docs().is_empty());
    for query_document_snapshot in query_snapshot.docs() {
        let data = query_document_snapshot.data::<HashMap<String, String>>()?;
        assert_eq!(data.get("k").map(String::as_str), Some("target"));
    }
    Ok(())
}
