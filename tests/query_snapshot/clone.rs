// since v2.1
#[test]
fn test_query_snapshot_clone() {
    // FIXME: add tests when QuerySnapshot can be constructed via the public Query API
    let _: fn(&bouzuya_firestore_client::QuerySnapshot) -> bouzuya_firestore_client::QuerySnapshot =
        bouzuya_firestore_client::QuerySnapshot::clone;
}
