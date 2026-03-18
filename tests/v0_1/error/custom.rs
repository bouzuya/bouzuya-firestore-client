#[test]
fn test_error_custom() {
    use bouzuya_firestore_client::Error;
    let _: Error = Error::custom("custom error1");
    let _: Error = Error::custom(std::io::Error::other("custom error2"));
}
