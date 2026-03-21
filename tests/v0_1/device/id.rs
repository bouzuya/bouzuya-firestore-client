#[test]
fn test_device_id() {
    use bouzuya_firestore_client::Device;
    let device = Device::new("device1".to_string(), "My Device".to_string());
    assert_eq!(device.id(), "device1");
}
