#[test]
fn test_timestamp_seconds() {
    use bouzuya_firestore_client::Timestamp;
    assert_eq!(Timestamp::from_millis(0).seconds(), 0_i64);
    assert_eq!(Timestamp::from_millis(1_000).seconds(), 1_i64);
    assert_eq!(Timestamp::from_millis(1_500).seconds(), 1_i64);
    assert_eq!(Timestamp::from_millis(-1_500).seconds(), -2_i64);
}
