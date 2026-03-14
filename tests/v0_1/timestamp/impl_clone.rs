#[test]
fn test_timestamp_clone() {
    fn assert_fn<T: Clone>() {}
    assert_fn::<bouzuya_firestore_client::Timestamp>();
}
