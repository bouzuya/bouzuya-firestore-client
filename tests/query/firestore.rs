// since v2.2
#[tokio::test]
async fn test_query_firestore() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let query = firestore.collection("rooms")?.limit(1);
    let _: &Firestore = query.firestore();
    Ok(())
}
