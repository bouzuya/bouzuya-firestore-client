// since v0.1 -> v4.0 (breaking change)
#[test]
fn test_timestamp_from_millis() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Timestamp;
    assert_eq!(Timestamp::from_millis(0)?.to_millis(), 0);
    assert_eq!(Timestamp::from_millis(1_000)?.to_millis(), 1_000);
    assert_eq!(Timestamp::from_millis(1_500)?.to_millis(), 1_500);
    assert_eq!(Timestamp::from_millis(-1)?.to_millis(), -1);
    assert_eq!(Timestamp::from_millis(-1_000)?.to_millis(), -1_000);
    assert_eq!(Timestamp::from_millis(-1_500)?.to_millis(), -1_500);
    assert_eq!(
        format!("{:?}", Timestamp::from_millis(-1)?),
        "Timestamp { seconds: -1, nanos: 999000000 }"
    );
    assert_eq!(
        format!("{:?}", Timestamp::from_millis(-1_000)?),
        "Timestamp { seconds: -1, nanos: 0 }"
    );
    assert_eq!(
        format!("{:?}", Timestamp::from_millis(-1_500)?),
        "Timestamp { seconds: -2, nanos: 500000000 }"
    );
    Ok(())
}

// since v4.0
#[test]
fn test_timestamp_from_millis_min_max() {
    use bouzuya_firestore_client::Timestamp;
    // min: 0001-01-01T00:00:00Z
    let min_millis: i64 = -62_135_596_800_000;
    // max: 9999-12-31T23:59:59.999Z
    let max_millis: i64 = 253_402_300_799_999;
    assert!(Timestamp::from_millis(min_millis).is_ok());
    assert!(Timestamp::from_millis(max_millis).is_ok());
    assert!(Timestamp::from_millis(min_millis - 1).is_err());
    assert!(Timestamp::from_millis(max_millis + 1).is_err());
}
