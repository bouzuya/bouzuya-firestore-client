#[test]
fn test_transaction_options_read_time() {
    use bouzuya_firestore_client::Timestamp;
    use bouzuya_firestore_client::TransactionOptions;
    let read_time = Timestamp::from_millis(1_000);
    let options = TransactionOptions {
        max_attempts: None,
        read_only: None,
        read_time: Some(read_time),
    };
    assert_eq!(options.read_time, Some(read_time));
}
