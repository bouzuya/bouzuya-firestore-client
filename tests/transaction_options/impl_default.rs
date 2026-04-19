// since v0.1
#[test]
fn test_transaction_options_default() {
    use bouzuya_firestore_client::TransactionOptions;
    fn assert_fn<T: Default>() {}
    assert_fn::<TransactionOptions>();
    let options = TransactionOptions::default();
    assert_eq!(options.max_attempts, None);
    assert_eq!(options.read_only, None);
    assert_eq!(options.read_time, None);
}
