// since v2.2
#[test]
fn test_query_where_() {
    fn _check(
        query: bouzuya_firestore_client::Query,
        filter: bouzuya_firestore_client::Filter,
    ) -> bouzuya_firestore_client::Query {
        query.r#where(filter)
    }
}
