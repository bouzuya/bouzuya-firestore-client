// since v2.2
#[test]
fn test_filter_where_() {
    fn _with_string(f: String, op: &str, v: i64) {
        use bouzuya_firestore_client::Filter;
        let _ = Filter::r#where(f, op, v);
    }
    fn _with_field_path(f: bouzuya_firestore_client::FieldPath, op: &str, v: i64) {
        use bouzuya_firestore_client::Filter;
        let _ = Filter::r#where(f, op, v);
    }
}
