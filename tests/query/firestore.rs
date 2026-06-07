// since v3.0 -> v4 (breaking change)
#[tokio::test]
async fn test_query_firestore() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let query = firestore.collection("rooms")?.limit(1)?;
    let actual: Firestore = query.firestore();
    assert_eq!(actual, firestore);
    Ok(())
}
