#[test]
fn test_set_options_import() {
    use bouzuya_firestore_client::SetOptions;
    let _: Option<SetOptions> = None;
}

#[test]
fn test_default() {
    use bouzuya_firestore_client::SetOptions;
    let set_options = SetOptions::default();
    assert!(set_options.merge.is_none());
    assert!(set_options.merge_fields.is_none());
}

#[test]
fn test_merge() {
    use bouzuya_firestore_client::SetOptions;
    let set_options = SetOptions {
        merge: Some(true),
        merge_fields: None,
    };
    assert_eq!(set_options.merge, Some(true));
}

#[test]
fn test_merge_fields() {
    use bouzuya_firestore_client::SetOptions;
    let set_options = SetOptions {
        merge: None,
        merge_fields: Some(vec!["field1".to_string()]),
    };
    assert_eq!(set_options.merge_fields, Some(vec!["field1".to_string()]));
}
