// since v2.2
#[test]
fn test_collection_reference_select() {
    fn _check(
        collection_reference: bouzuya_firestore_client::CollectionReference,
        field_path: bouzuya_firestore_client::FieldPath,
    ) -> Result<bouzuya_firestore_client::Query, bouzuya_firestore_client::Error> {
        collection_reference.select([field_path])
    }
}

// since v2.2
#[tokio::test]
#[serial_test::serial]
async fn test_collection_reference_select_get() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-collection-reference-select")?;
    for (a, b) in [(1_i64, 10_i64), (2_i64, 20_i64)] {
        collection_reference
            .add(
                [("a".to_owned(), a), ("b".to_owned(), b)]
                    .into_iter()
                    .collect::<HashMap<String, i64>>(),
            )
            .await?;
    }
    let query_snapshot = collection_reference.select(["a"])?.limit(10)?.get().await?;
    assert!(!query_snapshot.docs().is_empty());
    for query_document_snapshot in query_snapshot.docs() {
        let data = query_document_snapshot.data::<HashMap<String, i64>>()?;
        assert!(data.contains_key("a"));
        assert!(!data.contains_key("b"));
    }
    Ok(())
}
