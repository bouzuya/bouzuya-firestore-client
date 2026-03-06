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
fn test_timestamp_debug() {
    use bouzuya_firestore_client::Timestamp;
    fn assert_fn<T: std::fmt::Debug>() {}
    assert_fn::<Timestamp>();
    let t = Timestamp::from_millis(1_500);
    assert_eq!(
        format!("{:?}", t),
        "Timestamp { seconds: 1, nanos: 500000000 }"
    );
}

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

#[test]
fn test_timestamp_from_millis() {
    use bouzuya_firestore_client::Timestamp;
    assert_eq!(Timestamp::from_millis(0).to_millis(), 0);
    assert_eq!(Timestamp::from_millis(1_000).to_millis(), 1_000);
    assert_eq!(Timestamp::from_millis(1_500).to_millis(), 1_500);
    assert_eq!(Timestamp::from_millis(-1).to_millis(), -1);
    assert_eq!(Timestamp::from_millis(-1_000).to_millis(), -1_000);
    assert_eq!(Timestamp::from_millis(-1_500).to_millis(), -1_500);
    assert_eq!(
        format!("{:?}", Timestamp::from_millis(-1)),
        "Timestamp { seconds: -1, nanos: 999000000 }"
    );
    assert_eq!(
        format!("{:?}", Timestamp::from_millis(-1_000)),
        "Timestamp { seconds: -1, nanos: 0 }"
    );
    assert_eq!(
        format!("{:?}", Timestamp::from_millis(-1_500)),
        "Timestamp { seconds: -2, nanos: 500000000 }"
    );
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
    use bouzuya_firestore_client::Timestamp;
    fn assert_fn<T: Ord>() {}
    assert_fn::<Timestamp>();
    let t1 = Timestamp::from_millis(1_000);
    let t2 = Timestamp::from_millis(2_000);
    assert!(t1 < t2);
    assert!(t2 > t1);
    assert_eq!(t1.cmp(&t1), std::cmp::Ordering::Equal);
}

#[test]
fn test_timestamp_to_millis() {
    use bouzuya_firestore_client::Timestamp;
    assert_eq!(Timestamp::from_millis(0).to_millis(), 0);
    assert_eq!(Timestamp::from_millis(1_000).to_millis(), 1_000);
    assert_eq!(Timestamp::from_millis(1_500).to_millis(), 1_500);
}
