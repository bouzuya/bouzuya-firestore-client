// since v2.2
#[test]
fn test_collection_reference_start_after() {
    fn _check<I>(
        collection_reference: bouzuya_firestore_client::CollectionReference,
        values: I,
    ) -> Result<bouzuya_firestore_client::Query, bouzuya_firestore_client::Error>
    where
        I: IntoIterator,
        I::Item: serde::Serialize,
    {
        collection_reference.start_after(values)
    }
}
