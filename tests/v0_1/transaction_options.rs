#[test]
fn test_transaction_options_import() {
    use bouzuya_firestore_client::TransactionOptions;
    let _: Option<TransactionOptions> = None;
}

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
