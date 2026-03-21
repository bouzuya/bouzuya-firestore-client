#[test]
fn test_device_eq() {
    use bouzuya_firestore_client::Device;
    fn assert_fn<T: Eq>() {}
    assert_fn::<Device>();
    let device1 = Device::new("device1".to_string(), "My Device".to_string());
    let device2 = Device::new("device1".to_string(), "My Device".to_string());
    let device3 = Device::new("device2".to_string(), "Other Device".to_string());
    assert_eq!(device1, device2);
    assert_ne!(device1, device3);
}
