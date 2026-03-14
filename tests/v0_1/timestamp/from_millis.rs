#[test]
fn test_timestamp_from_millis() {
    use bouzuya_firestore_client::Timestamp;
    assert_eq!(Timestamp::from_millis(0).to_millis(), 0);
    assert_eq!(Timestamp::from_millis(1_000).to_millis(), 1_000);
    assert_eq!(Timestamp::from_millis(1_500).to_millis(), 1_500);
    assert_eq!(Timestamp::from_millis(-1).to_millis(), -1);
    assert_eq!(Timestamp::from_millis(-1_000).to_millis(), -1_000);
    assert_eq!(Timestamp::from_millis(-1_500).to_millis(), -1_500);
    assert_eq!(
        format!("{:?}", Timestamp::from_millis(-1)),
        "Timestamp { seconds: -1, nanos: 999000000 }"
    );
    assert_eq!(
        format!("{:?}", Timestamp::from_millis(-1_000)),
        "Timestamp { seconds: -1, nanos: 0 }"
    );
    assert_eq!(
        format!("{:?}", Timestamp::from_millis(-1_500)),
        "Timestamp { seconds: -2, nanos: 500000000 }"
    );
}
