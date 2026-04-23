// since v2.1
#[tokio::test]
async fn test_firestore_impl_eq() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;

    fn assert_impl<T: Eq>() {}
    assert_impl::<Firestore>();

    let firestore = Firestore::new(FirestoreOptions::default())?;
    assert_eq!(firestore, firestore.clone());
    let firestore2 = Firestore::new(FirestoreOptions::default())?;
    assert_ne!(firestore, firestore2);
    Ok(())
}
