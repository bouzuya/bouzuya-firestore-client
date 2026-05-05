// since v3.1
#[test]
fn test_collection_group_clone() {
    use bouzuya_firestore_client::CollectionGroup;

    fn assert_impl<T: Clone>() {}
    assert_impl::<CollectionGroup>();
}
