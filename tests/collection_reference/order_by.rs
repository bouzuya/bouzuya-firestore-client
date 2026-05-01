// since v2.2
#[test]
fn test_collection_reference_order_by() {
    fn _check(
        collection_reference: bouzuya_firestore_client::CollectionReference,
        field_path: bouzuya_firestore_client::FieldPath,
    ) -> Result<bouzuya_firestore_client::Query, bouzuya_firestore_client::Error> {
        collection_reference.order_by(field_path, "asc")
    }
}

// since v2.2
#[tokio::test]
async fn test_collection_reference_order_by_invalid_direction() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    let result = collection_reference.order_by("n", "ascending");
    assert!(result.is_err());
    Ok(())
}

// since v2.2
#[tokio::test]
#[serial_test::serial]
async fn test_collection_reference_order_by_get_asc() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-collection-reference-order-by-asc")?;
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
    assert!(!query_snapshot.docs().is_empty());
    let mut prev: Option<i64> = None;
    for doc in query_snapshot.docs() {
        let data = doc.data::<HashMap<String, i64>>()?;
        let n = data.get("n").copied();
        let n = n.ok_or_else(|| anyhow::anyhow!("n missing"))?;
        if let Some(p) = prev {
            assert!(p <= n, "expected ascending order, got {} after {}", n, p);
        }
        prev = Some(n);
    }
    Ok(())
}

// since v2.2
#[tokio::test]
#[serial_test::serial]
async fn test_collection_reference_order_by_get_desc() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-collection-reference-order-by-desc")?;
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
    assert!(!query_snapshot.docs().is_empty());
    let mut prev: Option<i64> = None;
    for doc in query_snapshot.docs() {
        let data = doc.data::<HashMap<String, i64>>()?;
        let n = data.get("n").copied();
        let n = n.ok_or_else(|| anyhow::anyhow!("n missing"))?;
        if let Some(p) = prev {
            assert!(p >= n, "expected descending order, got {} after {}", n, p);
        }
        prev = Some(n);
    }
    Ok(())
}

// since v2.2
#[tokio::test]
#[serial_test::serial]
async fn test_collection_reference_order_by_append() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    #[derive(serde::Deserialize, serde::Serialize)]
    struct Doc {
        a: i64,
        b: i64,
    }
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("test-collection-reference-order-by-append")?;
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
    assert!(!query_snapshot.docs().is_empty());
    let mut prev: Option<(i64, i64)> = None;
    for doc in query_snapshot.docs() {
        let data = doc.data::<Doc>()?;
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
