// since v2.1
#[tokio::test]
async fn test_firestore_impl_debug() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;

    fn assert_impl<T: std::fmt::Debug>() {}
    assert_impl::<Firestore>();

    let firestore = Firestore::new(FirestoreOptions::default())?;
    let debug_str = format!("{:?}", firestore);
    assert!(debug_str.contains("Firestore"));
    Ok(())
}
