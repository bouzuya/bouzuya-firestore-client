// since v2.1
#[tokio::test]
async fn test_document_reference_impl_debug() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::DocumentReference;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;

    fn assert_impl<T: std::fmt::Debug>() {}
    assert_impl::<DocumentReference>();

    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let debug_str = format!("{:?}", document_ref);
    assert!(debug_str.contains("DocumentReference"));
    Ok(())
}
