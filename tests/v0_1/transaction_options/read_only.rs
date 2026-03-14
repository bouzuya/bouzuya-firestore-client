#[test]
fn test_transaction_options_read_only() {
    use bouzuya_firestore_client::TransactionOptions;
    let options = TransactionOptions {
        max_attempts: None,
        read_only: Some(true),
        read_time: None,
    };
    assert_eq!(options.read_only, Some(true));
}
