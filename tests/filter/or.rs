// since v3.0
#[test]
fn test_filter_or() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    use bouzuya_firestore_client::Filter;
    let f1 = Filter::r#where(FieldPath::new(["age"])?, "==", 30_i64)?;
    let f2 = Filter::r#where(FieldPath::new(["name"])?, "==", "Alice")?;
    let _: Filter = Filter::or([f1, f2]);
    Ok(())
}

// since v3.0
#[tokio::test]
#[serial_test::serial]
async fn test_filter_or_get() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Filter;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    #[derive(serde::Deserialize, serde::Serialize)]
    struct Doc {
        k: String,
        n: i64,
    }
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-filter-or-get")?;
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
    let filter = Filter::or([f1, f2]);
    let query_snapshot = collection_reference.r#where(filter)?.get().await?;
    assert!(!query_snapshot.docs().is_empty());
    for query_document_snapshot in query_snapshot.docs() {
        let data = query_document_snapshot.data::<Doc>()?;
        assert!(data.n == 1 || data.k == "a");
    }
    Ok(())
}
