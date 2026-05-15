// since v0.1
#[tokio::test]
async fn test_firestore_impl_clone() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;

    fn assert_impl<T: Clone>() {}
    assert_impl::<Firestore>();

    let firestore = Firestore::new(FirestoreOptions::default())?;
    let cloned = firestore.clone();
    assert_eq!(firestore, cloned);
    Ok(())
}
