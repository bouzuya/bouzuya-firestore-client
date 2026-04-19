// since v0.1
#[tokio::test]
async fn test_document_reference_clone() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::DocumentReference;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_ref = firestore.doc("rooms/roomA")?;
    let cloned = document_ref.clone();
    assert_eq!(cloned.id().to_string(), "roomA");

    fn assert_fn<T: Clone>() {}
    assert_fn::<DocumentReference>();
    Ok(())
}
