#[test]
fn test_timestamp_ord() {
    use bouzuya_firestore_client::Timestamp;
    fn assert_fn<T: Ord>() {}
    assert_fn::<Timestamp>();
    let t1 = Timestamp::from_millis(1_000);
    let t2 = Timestamp::from_millis(2_000);
    assert!(t1 < t2);
    assert!(t2 > t1);
    assert_eq!(t1.cmp(&t1), std::cmp::Ordering::Equal);
}
