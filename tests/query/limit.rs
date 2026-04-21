// FIXME
// since v2.1
#[tokio::test]
async fn test_query_limit() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use bouzuya_firestore_client::Query;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let collection_ref = firestore.collection("rooms")?;
    let _query: Query = collection_ref.limit(10);
    Ok(())
}
