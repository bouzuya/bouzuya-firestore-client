#[test]
fn test_timestamp_eq() {
    use bouzuya_firestore_client::Timestamp;
    fn assert_fn<T: Eq>() {}
    assert_fn::<Timestamp>();
    let t1 = Timestamp::from_millis(1_000);
    let t2 = Timestamp::from_millis(1_000);
    let t3 = Timestamp::from_millis(2_000);
    assert_eq!(t1, t2);
    assert_ne!(t1, t3);
}
