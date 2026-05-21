/// Options for constructing a [`Firestore`](crate::Firestore) client.
///
/// Every field is optional. [`FirestoreOptions::default`] leaves both
/// unset, so each is resolved from the environment (or a built-in
/// default); set a field explicitly to override that resolution. The
/// resolution rules are documented on each field below.
///
/// # Examples
///
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use bouzuya_firestore_client::Firestore;
/// use bouzuya_firestore_client::FirestoreOptions;
///
/// // Resolve everything from the environment.
/// let firestore = Firestore::new(FirestoreOptions::default())?;
///
/// // Set both fields explicitly.
/// let firestore = Firestore::new(FirestoreOptions {
///     database_id: Some("my-database".to_owned()),
///     project_id: Some("my-project".to_owned()),
/// })?;
/// # Ok(())
/// # }
/// ```
#[derive(Default)]
pub struct FirestoreOptions {
    /// The Firestore database ID.
    ///
    /// When `None`, the database ID `"(default)"` is used.
    pub database_id: Option<String>,
    /// The Google Cloud project ID.
    ///
    /// When `None`, it is read from the `GCLOUD_PROJECT` environment
    /// variable, then from `GOOGLE_CLOUD_PROJECT`.
    /// [`Firestore::new`](crate::Firestore::new) fails if the project ID
    /// cannot be resolved from any of these.
    pub project_id: Option<String>,
}
