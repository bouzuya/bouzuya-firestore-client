// since v2.2
#[test]
fn test_collection_reference_order_by() {
    fn _check(
        collection_reference: bouzuya_firestore_client::CollectionReference,
        field_path: bouzuya_firestore_client::FieldPath,
    ) -> Result<bouzuya_firestore_client::Query, bouzuya_firestore_client::Error> {
        collection_reference.order_by(field_path, "asc")
    }
}
