// since v2.1
#[test]
fn test_query_clone() {
    use bouzuya_firestore_client::Query;

    fn assert_impl<T: Clone>() {}
    assert_impl::<Query>();
}
