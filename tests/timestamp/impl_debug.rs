// since v0.1
#[test]
fn test_timestamp_debug() {
    use bouzuya_firestore_client::Timestamp;
    fn assert_fn<T: std::fmt::Debug>() {}
    assert_fn::<Timestamp>();
    let t = Timestamp::from_millis(1_500);
    assert_eq!(
        format!("{:?}", t),
        "Timestamp { seconds: 1, nanos: 500000000 }"
    );
}
