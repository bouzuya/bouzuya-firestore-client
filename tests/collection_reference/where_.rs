// since v2.2
#[test]
fn test_collection_reference_where_() {
    fn _check(
        collection_reference: bouzuya_firestore_client::CollectionReference,
        filter: bouzuya_firestore_client::Filter,
    ) -> bouzuya_firestore_client::Query {
        collection_reference.r#where(filter)
    }
}

// since v2.2
#[tokio::test]
#[serial_test::serial]
async fn test_collection_reference_where_get() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Filter;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-collection-reference-where")?;
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
    let filter = Filter::r#where("k", "==", "target".to_string())?;
    let query_snapshot = collection_reference.r#where(filter).get().await?;
    assert!(!query_snapshot.docs().is_empty());
    for query_document_snapshot in query_snapshot.docs() {
        let data = query_document_snapshot.data::<HashMap<String, String>>()?;
        assert_eq!(data.get("k").map(String::as_str), Some("target"));
    }
    Ok(())
}
