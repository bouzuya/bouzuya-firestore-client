// since v3.2
#[test]
fn test_timestamp_now() -> anyhow::Result<()> {
    use bouzuya_firestore_client::Timestamp;
    let before = i64::try_from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis(),
    )?;
    let now = Timestamp::now();
    let after = i64::try_from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_millis(),
    )?;
    assert!((before..=after).contains(&now.to_millis()));
    Ok(())
}
