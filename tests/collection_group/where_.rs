// since v3.1
#[test]
fn test_collection_group_where_() {
    fn _check(
        collection_group: bouzuya_firestore_client::CollectionGroup,
        filter: bouzuya_firestore_client::Filter,
    ) -> Result<bouzuya_firestore_client::Query, bouzuya_firestore_client::Error> {
        collection_group.r#where(filter)?;
        collection_group.r#where(("k", "==", 1_i64))
    }
}

// since v3.1
#[tokio::test]
#[serial_test::serial]
async fn test_collection_group_where_get() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Filter;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    // arrange
    let sub_collection_name = format!("collection-group-where-{}", uuid::Uuid::now_v7());
    let document_reference1 = firestore
        .collection("projects")?
        .add(HashMap::<String, String>::new())
        .await?;
    for k in ["target", "other"] {
        document_reference1
            .collection(&sub_collection_name)?
            .add(
                [("k".to_owned(), k.to_owned())]
                    .into_iter()
                    .collect::<HashMap<String, String>>(),
            )
            .await?;
    }
    let document_reference2 = firestore
        .collection("rooms")?
        .add(HashMap::<String, String>::new())
        .await?;
    for k in ["target", "other"] {
        document_reference2
            .collection(&sub_collection_name)?
            .add(
                [("k".to_owned(), k.to_owned())]
                    .into_iter()
                    .collect::<HashMap<String, String>>(),
            )
            .await?;
    }
    // act
    let collection_group = firestore.collection_group(&sub_collection_name)?;
    let filter = Filter::r#where("k", "==", "target".to_string())?;
    let query_snapshot = collection_group.r#where(filter)?.get().await?;
    // assert
    assert!(!query_snapshot.empty());
    for query_document_snapshot in query_snapshot {
        let data = query_document_snapshot.data::<HashMap<String, String>>()?;
        assert_eq!(data.get("k").map(String::as_str), Some("target"));
    }
    Ok(())
}

// since v3.1
#[tokio::test]
#[serial_test::serial]
async fn test_collection_group_where_tuple() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::collections::HashMap;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    // arrange
    let sub_collection_name = format!("collection-group-where-tuple-{}", uuid::Uuid::now_v7());
    let document_reference1 = firestore
        .collection("projects")?
        .add(HashMap::<String, String>::new())
        .await?;
    for k in ["target", "other"] {
        document_reference1
            .collection(&sub_collection_name)?
            .add(
                [("k".to_owned(), k.to_owned())]
                    .into_iter()
                    .collect::<HashMap<String, String>>(),
            )
            .await?;
    }
    let document_reference2 = firestore
        .collection("rooms")?
        .add(HashMap::<String, String>::new())
        .await?;
    for k in ["target", "other"] {
        document_reference2
            .collection(&sub_collection_name)?
            .add(
                [("k".to_owned(), k.to_owned())]
                    .into_iter()
                    .collect::<HashMap<String, String>>(),
            )
            .await?;
    }
    // act
    let collection_group = firestore.collection_group(&sub_collection_name)?;
    let query_snapshot = collection_group
        .r#where(("k", "==", "target".to_string()))?
        .get()
        .await?;
    // assert
    assert!(!query_snapshot.empty());
    for query_document_snapshot in query_snapshot {
        let data = query_document_snapshot.data::<HashMap<String, String>>()?;
        assert_eq!(data.get("k").map(String::as_str), Some("target"));
    }
    Ok(())
}
