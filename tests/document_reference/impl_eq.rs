// since v2.1
#[tokio::test]
async fn test_document_reference_impl_eq() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::DocumentReference;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;

    fn assert_impl<T: Eq>() {}
    assert_impl::<DocumentReference>();

    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_reference = firestore.doc("rooms/roomA")?;
    assert_eq!(document_reference, document_reference.clone());
    let doc_ref2 = firestore.doc("rooms/roomB")?;
    assert_ne!(document_reference, doc_ref2);
    Ok(())
}
