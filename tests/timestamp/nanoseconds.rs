// since v3.0
#[test]
fn test_timestamp_nanoseconds() {
    use bouzuya_firestore_client::Timestamp;
    assert_eq!(Timestamp::from_millis(0).nanoseconds(), 0_i32);
    assert_eq!(Timestamp::from_millis(1_000).nanoseconds(), 0_i32);
    assert_eq!(Timestamp::from_millis(1_500).nanoseconds(), 500_000_000_i32);
    assert_eq!(
        Timestamp::from_millis(-1_500).nanoseconds(),
        500_000_000_i32
    );
}
