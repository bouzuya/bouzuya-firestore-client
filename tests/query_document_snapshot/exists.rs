// since v2.1

#[test]
fn test_query_document_snapshot_exists() {
    // FIXME: add tests when QueryDocumentSnapshot can be obtained via the public Query API
    let _: fn(&bouzuya_firestore_client::QueryDocumentSnapshot) -> bool =
        bouzuya_firestore_client::QueryDocumentSnapshot::exists;
}
