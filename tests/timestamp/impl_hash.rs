// since v0.1
#[test]
fn test_timestamp_hash() {
    fn assert_fn<T: std::hash::Hash>() {}
    assert_fn::<bouzuya_firestore_client::Timestamp>();
}
