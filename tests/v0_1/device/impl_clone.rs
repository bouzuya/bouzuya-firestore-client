#[test]
fn test_device_clone() {
    fn assert_fn<T: Clone>() {}
    assert_fn::<bouzuya_firestore_client::Device>();
}
