// since v2.2
#[test]
fn test_filter_and() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    use bouzuya_firestore_client::Filter;
    let f1 = Filter::r#where(FieldPath::new(["age"])?, "==", 30_i64)?;
    let f2 = Filter::r#where(FieldPath::new(["name"])?, "==", "Alice")?;
    let _: Filter = Filter::and([f1, f2]);
    Ok(())
}

// since v2.2
#[tokio::test]
#[serial_test::serial]
async fn test_filter_and_get() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Filter;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    #[derive(serde::Deserialize, serde::Serialize)]
    struct Doc {
        k: String,
        n: i64,
    }
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-filter-and-get")?;
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
    let filter = Filter::and([f1, f2]);
    let query_snapshot = collection_reference.r#where(filter).get().await?;
    assert!(!query_snapshot.docs().is_empty());
    for query_document_snapshot in query_snapshot.docs() {
        let data = query_document_snapshot.data::<Doc>()?;
        assert_eq!(data.n, 1);
        assert_eq!(data.k, "a");
    }
    Ok(())
}
