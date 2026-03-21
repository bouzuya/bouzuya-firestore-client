#[test]
fn test_device_debug() {
    use bouzuya_firestore_client::Device;
    fn assert_fn<T: std::fmt::Debug>() {}
    assert_fn::<Device>();
    let device = Device::new("device1".to_string(), "My Device".to_string());
    assert_eq!(
        format!("{:?}", device),
        r#"Device { id: "device1", name: "My Device" }"#
    );
}
