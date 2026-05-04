// since v3.0
#[test]
fn test_eq() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    fn assert_impl<T: Eq>() {}
    assert_impl::<FieldPath>();
    let fp = FieldPath::new(["a"])?;
    assert!(fp == fp);
    Ok(())
}
