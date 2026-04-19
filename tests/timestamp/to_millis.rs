// since v0.1
#[test]
fn test_timestamp_to_millis() {
    use bouzuya_firestore_client::Timestamp;
    assert_eq!(Timestamp::from_millis(0).to_millis(), 0);
    assert_eq!(Timestamp::from_millis(1_000).to_millis(), 1_000);
    assert_eq!(Timestamp::from_millis(1_500).to_millis(), 1_500);
}
