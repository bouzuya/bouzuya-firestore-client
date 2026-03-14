#[test]
fn test_transaction_options_max_attempts() {
    use bouzuya_firestore_client::TransactionOptions;
    let options = TransactionOptions {
        max_attempts: Some(5),
        read_only: None,
        read_time: None,
    };
    assert_eq!(options.max_attempts, Some(5));
}
