// since v3.1
#[test]
fn test_collection_group_order_by() {
    fn _check(
        collection_group: bouzuya_firestore_client::CollectionGroup,
        field_path: bouzuya_firestore_client::FieldPath,
    ) -> Result<bouzuya_firestore_client::Query, bouzuya_firestore_client::Error> {
        collection_group.order_by(field_path, "asc")
    }
}

// since v3.1
#[tokio::test]
async fn test_collection_group_order_by_invalid_direction() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_group = firestore.collection_group("rooms")?;
    let result = collection_group.order_by("n", "ascending");
    assert!(result.is_err());
    Ok(())
}

// since v3.1
#[tokio::test]
#[serial_test::serial]
async fn test_collection_group_order_by_get_asc() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    // arrange
    let sub_collection_name = format!("collection-group-order-by-asc-{}", uuid::Uuid::now_v7());
    let document_reference1 = firestore
        .collection("projects")?
        .add(HashMap::<String, String>::new())
        .await?;
    for n in [3_i64, 1_i64, 2_i64] {
        document_reference1
            .collection(&sub_collection_name)?
            .add(
                [("n".to_owned(), n)]
                    .into_iter()
                    .collect::<HashMap<String, i64>>(),
            )
            .await?;
    }
    let document_reference2 = firestore
        .collection("rooms")?
        .add(HashMap::<String, String>::new())
        .await?;
    for n in [3_i64, 1_i64, 2_i64] {
        document_reference2
            .collection(&sub_collection_name)?
            .add(
                [("n".to_owned(), n)]
                    .into_iter()
                    .collect::<HashMap<String, i64>>(),
            )
            .await?;
    }
    // act
    let collection_group = firestore.collection_group(&sub_collection_name)?;
    let query_snapshot = collection_group.order_by("n", "asc")?.get().await?;
    // assert
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

// since v3.1
#[tokio::test]
#[serial_test::serial]
async fn test_collection_reference_order_by_get_desc() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    // arrange
    let sub_collection_name = format!("collection-group-order-by-desc-{}", uuid::Uuid::now_v7());
    let document_reference1 = firestore
        .collection("projects")?
        .add(HashMap::<String, String>::new())
        .await?;
    for n in [3_i64, 1_i64, 2_i64] {
        document_reference1
            .collection(&sub_collection_name)?
            .add(
                [("n".to_owned(), n)]
                    .into_iter()
                    .collect::<HashMap<String, i64>>(),
            )
            .await?;
    }
    let document_reference2 = firestore
        .collection("rooms")?
        .add(HashMap::<String, String>::new())
        .await?;
    for n in [3_i64, 1_i64, 2_i64] {
        document_reference2
            .collection(&sub_collection_name)?
            .add(
                [("n".to_owned(), n)]
                    .into_iter()
                    .collect::<HashMap<String, i64>>(),
            )
            .await?;
    }
    // act
    let collection_group = firestore.collection_group(&sub_collection_name)?;
    let query_snapshot = collection_group.order_by("n", "desc")?.get().await?;
    // assert
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

// since v3.1
#[tokio::test]
#[serial_test::serial]
async fn test_collection_group_order_by_append() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    #[derive(serde::Deserialize, serde::Serialize)]
    struct Doc {
        a: i64,
        b: i64,
    }
    let firestore = Firestore::new(FirestoreOptions::default())?;
    // arrange
    let sub_collection_name = format!("collection-group-order-by-append-{}", uuid::Uuid::now_v7());
    let document_reference1 = firestore
        .collection("projects")?
        .add(HashMap::<String, String>::new())
        .await?;
    for (a, b) in [
        (2_i64, 1_i64),
        (1_i64, 2_i64),
        (1_i64, 1_i64),
        (2_i64, 2_i64),
    ] {
        document_reference1
            .collection(&sub_collection_name)?
            .add(Doc { a, b })
            .await?;
    }
    let document_reference2 = firestore
        .collection("rooms")?
        .add(HashMap::<String, String>::new())
        .await?;
    for (a, b) in [
        (2_i64, 1_i64),
        (1_i64, 2_i64),
        (1_i64, 1_i64),
        (2_i64, 2_i64),
    ] {
        document_reference2
            .collection(&sub_collection_name)?
            .add(Doc { a, b })
            .await?;
    }
    // act
    let collection_group = firestore.collection_group(&sub_collection_name)?;
    let query_snapshot = collection_group
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
