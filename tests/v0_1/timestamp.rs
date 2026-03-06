#[test]
fn test_timestamp_clone() {
    fn assert_fn<T: Clone>() {}
    assert_fn::<bouzuya_firestore_client::Timestamp>();
}

#[test]
fn test_timestamp_copy() {
    fn assert_fn<T: Copy>() {}
    assert_fn::<bouzuya_firestore_client::Timestamp>();
}

#[test]
fn test_timestamp_eq() {
    fn assert_fn<T: Eq>() {}
    assert_fn::<bouzuya_firestore_client::Timestamp>();
}

#[test]
fn test_timestamp_hash() {
    fn assert_fn<T: std::hash::Hash>() {}
    assert_fn::<bouzuya_firestore_client::Timestamp>();
}

#[test]
fn test_timestamp_import() {
    use bouzuya_firestore_client::Timestamp;
    let _: Option<Timestamp> = None;
}

#[test]
fn test_timestamp_ord() {
    fn assert_fn<T: Ord>() {}
    assert_fn::<bouzuya_firestore_client::Timestamp>();
}
