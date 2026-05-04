// since v3.0
#[test]
fn test_equal() -> anyhow::Result<()> {
    use bouzuya_firestore_client::FieldPath;
    let fp1 = FieldPath::new(["a"])?;
    let fp2 = FieldPath::new(["a"])?;
    assert!(fp1 == fp2);
    Ok(())
}
