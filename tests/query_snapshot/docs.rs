// since v2.1
#[test]
fn test_query_snapshot_docs() {
    // FIXME: add tests when QuerySnapshot can be constructed via the public Query API
    let _: fn(
        &bouzuya_firestore_client::QuerySnapshot,
    ) -> Vec<bouzuya_firestore_client::QueryDocumentSnapshot> =
        bouzuya_firestore_client::QuerySnapshot::docs;
}
