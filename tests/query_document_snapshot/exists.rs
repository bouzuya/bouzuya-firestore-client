// since v2.1

#[test]
fn test_query_document_snapshot_exists() {
    let _: fn(&bouzuya_firestore_client::QueryDocumentSnapshot) -> bool =
        bouzuya_firestore_client::QueryDocumentSnapshot::exists;
}
