// since v2.1
#[tokio::test]
async fn test_document_reference_impl_eq() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::DocumentReference;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;

    fn assert_impl<T: Eq>() {}
    assert_impl::<DocumentReference>();

    let firestore = Firestore::new(FirestoreOptions::default())?;
    let doc_ref = firestore.doc("rooms/roomA")?;
    assert_eq!(doc_ref, doc_ref.clone());
    let doc_ref2 = firestore.doc("rooms/roomB")?;
    assert_ne!(doc_ref, doc_ref2);
    Ok(())
}
