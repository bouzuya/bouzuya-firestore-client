#[test]
fn test_document_reference_import() {
    use bouzuya_firestore_client::DocumentReference;
    let _: Option<DocumentReference> = None;
}

#[test]
fn test_document_reference_id() -> Result<(), bouzuya_firestore_client::Error> {
    use bouzuya_firestore_client::DocumentPath;
    use bouzuya_firestore_client::Firestore;
    use bouzuya_firestore_client::FirestoreOptions;
    use std::str::FromStr as _;
    let firestore = Firestore::new(FirestoreOptions::default())?;
    let document_path = DocumentPath::from_str("rooms/roomA")?;
    let document_ref = firestore.doc(document_path);
    assert_eq!(document_ref.id().to_string(), "roomA");
    Ok(())
}
