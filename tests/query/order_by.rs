// since v3.0
#[test]
fn test_query_order_by() {
    fn _check(
        query: bouzuya_firestore_client::Query,
        field_path: bouzuya_firestore_client::FieldPath,
    ) -> Result<bouzuya_firestore_client::Query, bouzuya_firestore_client::Error> {
        query.order_by(field_path, "asc")
    }
}

// since v3.0
#[tokio::test]
async fn test_query_order_by_invalid_direction() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    let query = collection_reference.limit(1)?;
    let result = query.order_by("n", "ascending");
    assert!(result.is_err());
    Ok(())
}

// since v3.0
#[tokio::test]
#[serial_test::serial]
async fn test_query_order_by_get_asc() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-query-order-by-asc")?;
    for n in [3_i64, 1_i64, 2_i64] {
        collection_reference
            .add(
                [("n".to_owned(), n)]
                    .into_iter()
                    .collect::<HashMap<String, i64>>(),
            )
            .await?;
    }
    let query_snapshot = collection_reference.order_by("n", "asc")?.get().await?;
    assert!(!query_snapshot.empty());
    let mut prev: Option<i64> = None;
    for query_document_snapshot in query_snapshot {
        let data = query_document_snapshot.data::<HashMap<String, i64>>()?;
        let n = data.get("n").copied();
        let n = n.ok_or_else(|| anyhow::anyhow!("n missing"))?;
        if let Some(p) = prev {
            assert!(p <= n, "expected ascending order, got {} after {}", n, p);
        }
        prev = Some(n);
    }
    Ok(())
}

// since v3.0
#[tokio::test]
#[serial_test::serial]
async fn test_query_order_by_get_desc() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-query-order-by-desc")?;
    for n in [3_i64, 1_i64, 2_i64] {
        collection_reference
            .add(
                [("n".to_owned(), n)]
                    .into_iter()
                    .collect::<HashMap<String, i64>>(),
            )
            .await?;
    }
    let query_snapshot = collection_reference.order_by("n", "desc")?.get().await?;
    assert!(!query_snapshot.empty());
    let mut prev: Option<i64> = None;
    for query_document_snapshot in query_snapshot {
        let data = query_document_snapshot.data::<HashMap<String, i64>>()?;
        let n = data.get("n").copied();
        let n = n.ok_or_else(|| anyhow::anyhow!("n missing"))?;
        if let Some(p) = prev {
            assert!(p >= n, "expected descending order, got {} after {}", n, p);
        }
        prev = Some(n);
    }
    Ok(())
}

// since v3.0
#[tokio::test]
#[serial_test::serial]
async fn test_query_order_by_append() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    #[derive(serde::Deserialize, serde::Serialize)]
    struct Doc {
        a: i64,
        b: i64,
    }
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-query-order-by-append")?;
    for (a, b) in [
        (2_i64, 1_i64),
        (1_i64, 2_i64),
        (1_i64, 1_i64),
        (2_i64, 2_i64),
    ] {
        collection_reference.add(Doc { a, b }).await?;
    }
    let query_snapshot = collection_reference
        .order_by("a", "asc")?
        .order_by("b", "asc")?
        .get()
        .await?;
    assert!(!query_snapshot.empty());
    let mut prev: Option<(i64, i64)> = None;
    for query_document_snapshot in query_snapshot {
        let data = query_document_snapshot.data::<Doc>()?;
        let curr = (data.a, data.b);
        if let Some(p) = prev {
            assert!(
                p <= curr,
                "expected (a, b) ascending, got {:?} after {:?}",
                curr,
                p
            );
        }
        prev = Some(curr);
    }
    Ok(())
}
