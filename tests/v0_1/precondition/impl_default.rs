#[test]
fn test_default() {
    use bouzuya_firestore_client::Precondition;
    let precondition = Precondition::default();
    assert!(precondition.exists.is_none());
    assert!(precondition.last_update_time.is_none());
}
