// since v2.1
#[test]
fn test_document_snapshot_clone() {
    // FIXME: add tests when DocumentSnapshot can be obtained via the public API
    let _: fn(
        &bouzuya_firestore_client::DocumentSnapshot,
    ) -> bouzuya_firestore_client::DocumentSnapshot =
        bouzuya_firestore_client::DocumentSnapshot::clone;
}
