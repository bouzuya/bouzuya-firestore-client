// since v0.1
#[test]
fn test_timestamp_copy() {
    fn assert_fn<T: Copy>() {}
    assert_fn::<bouzuya_firestore_client::Timestamp>();
}
