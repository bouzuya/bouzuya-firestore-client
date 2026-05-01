// since v2.2
#[test]
fn test_query_order_by() {
    fn _check(
        query: bouzuya_firestore_client::Query,
        field_path: bouzuya_firestore_client::FieldPath,
    ) -> Result<bouzuya_firestore_client::Query, bouzuya_firestore_client::Error> {
        query.order_by(field_path, "asc")
    }
}
