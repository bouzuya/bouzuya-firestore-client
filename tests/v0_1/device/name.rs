#[test]
fn test_device_name() {
    use bouzuya_firestore_client::Device;
    let device = Device::new("device1".to_string(), "My Device".to_string());
    assert_eq!(device.name(), "My Device");
}
