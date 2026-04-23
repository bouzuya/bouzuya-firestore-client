// since v2.1
#[tokio::test]
async fn test_query_impl_partial_eq() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Query;

    fn assert_impl<T: std::cmp::PartialEq>() {}
    assert_impl::<Query>();

    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("rooms")?;
    let query: Query = collection_ref.offset(0);
    assert_eq!(query, query.clone());
    let query2: Query = collection_ref.offset(1);
    assert_ne!(query, query2);
    Ok(())
}
