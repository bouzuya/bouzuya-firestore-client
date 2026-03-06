#[test]
fn test_timestamp_clone() {
    fn assert_fn<T: Clone>() {}
    assert_fn::<bouzuya_firestore_client::Timestamp>();
}

#[test]
fn test_timestamp_import() {
    use bouzuya_firestore_client::Timestamp;
    let _: Option<Timestamp> = None;
}
