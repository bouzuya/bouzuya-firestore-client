// since v2.1
#[test]
fn test_query_document_snapshot_clone() {
    // FIXME: add tests when QueryDocumentSnapshot can be obtained via the public Query API
    let _: fn(
        &bouzuya_firestore_client::QueryDocumentSnapshot,
    ) -> bouzuya_firestore_client::QueryDocumentSnapshot =
        bouzuya_firestore_client::QueryDocumentSnapshot::clone;
}
