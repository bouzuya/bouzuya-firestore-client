// since v2.2
#[test]
fn test_query_where_() {
    fn _check(
        query: bouzuya_firestore_client::Query,
        filter: bouzuya_firestore_client::Filter,
    ) -> Result<bouzuya_firestore_client::Query, bouzuya_firestore_client::Error> {
        query.r#where(filter)?;
        query.r#where(("k", "==", 1_i64))
    }
}

// since v2.2
#[tokio::test]
#[serial_test::serial]
async fn test_query_where_append() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Filter;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    #[derive(serde::Deserialize, serde::Serialize)]
    struct Doc {
        k: String,
        n: i64,
    }
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-query-where-append")?;
    for (n, k) in [(1_i64, "a"), (2_i64, "a"), (1_i64, "b"), (2_i64, "b")] {
        collection_reference
            .add(Doc {
                k: k.to_string(),
                n,
            })
            .await?;
    }
    let f1 = Filter::r#where("n", "==", 1_i64)?;
    let f2 = Filter::r#where("k", "==", "a".to_string())?;
    let query_snapshot = collection_reference.r#where(f1)?.r#where(f2)?.get().await?;
    assert!(!query_snapshot.docs().is_empty());
    for query_document_snapshot in query_snapshot.docs() {
        let data = query_document_snapshot.data::<Doc>()?;
        assert_eq!(data.n, 1);
        assert_eq!(data.k, "a");
    }
    Ok(())
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
    let filter = Filter::r#where("k", "==", "target".to_string())?;
    let query = collection_reference.r#where(filter)?;
    let query_snapshot = query.get().await?;
    assert!(!query_snapshot.docs().is_empty());
    for query_document_snapshot in query_snapshot.docs() {
        let data = query_document_snapshot.data::<HashMap<String, String>>()?;
        assert_eq!(data.get("k").map(String::as_str), Some("target"));
    }
    Ok(())
}

// since v2.2
#[tokio::test]
#[serial_test::serial]
async fn test_query_where_tuple() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Filter;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    #[derive(serde::Deserialize, serde::Serialize)]
    struct Doc {
        k: String,
        n: i64,
    }
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-query-where-tuple")?;
    for (n, k) in [(1_i64, "a"), (2_i64, "a"), (1_i64, "b"), (2_i64, "b")] {
        collection_reference
            .add(Doc {
                k: k.to_string(),
                n,
            })
            .await?;
    }
    let f1 = Filter::r#where("n", "==", 1_i64)?;
    let query_snapshot = collection_reference
        .r#where(f1)?
        .r#where(("k", "==", "a".to_string()))?
        .get()
        .await?;
    assert!(!query_snapshot.docs().is_empty());
    for query_document_snapshot in query_snapshot.docs() {
        let data = query_document_snapshot.data::<Doc>()?;
        assert_eq!(data.n, 1);
        assert_eq!(data.k, "a");
    }
    Ok(())
}
