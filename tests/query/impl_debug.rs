// since v2.1
#[tokio::test]
async fn test_query_impl_debug() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Query;

    fn assert_impl<T: std::fmt::Debug>() {}
    assert_impl::<Query>();

    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_reference = firestore.collection("rooms")?;
    let query: Query = collection_reference.offset(0);
    let debug_str = format!("{:?}", query);
    assert!(debug_str.contains("Query"));
    Ok(())
}
