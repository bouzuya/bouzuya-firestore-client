// since v4.0
#[test]
fn test_timestamp_new() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Timestamp;
    // valid: typical
    let t = Timestamp::new(1_234_567_890, 123_456_789)?;
    assert_eq!(t.seconds(), 1_234_567_890);
    assert_eq!(t.nanoseconds(), 123_456_789);

    // valid: boundaries
    // min: 0001-01-01T00:00:00Z
    let min_seconds: i64 = -62_135_596_800;
    // max: 9999-12-31T23:59:59Z
    let max_seconds: i64 = 253_402_300_799;
    assert!(Timestamp::new(min_seconds, 0).is_ok());
    assert!(Timestamp::new(min_seconds, 999_999_999).is_ok());
    assert!(Timestamp::new(max_seconds, 0).is_ok());
    assert!(Timestamp::new(max_seconds, 999_999_999).is_ok());

    // invalid: seconds out of range
    assert!(Timestamp::new(min_seconds - 1, 0).is_err());
    assert!(Timestamp::new(max_seconds + 1, 0).is_err());

    // invalid: nanoseconds out of range
    assert!(Timestamp::new(0, -1).is_err());
    assert!(Timestamp::new(0, 1_000_000_000).is_err());

    Ok(())
}
