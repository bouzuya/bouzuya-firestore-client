//! A client for [Cloud Firestore].
//!
//! This crate provides an asynchronous client for reading and writing data in
//! a Cloud Firestore database. Its API mirrors the [Node.js Firestore client],
//! so its types and method names should feel familiar if you have used that
//! library.
//!
//! # Getting started
//!
//! [`Firestore`] is the entry point. Create one with [`Firestore::new`],
//! passing [`FirestoreOptions`] to configure the project and database (both
//! are resolved from the environment when left unset). From a [`Firestore`]
//! you can obtain:
//!
//! - a [`DocumentReference`] via [`Firestore::doc`] — a reference to a single
//!   document that can be read ([`get`](DocumentReference::get)) and written
//!   ([`create`](DocumentReference::create), [`set`](DocumentReference::set),
//!   [`update`](DocumentReference::update),
//!   [`delete`](DocumentReference::delete)),
//! - a [`CollectionReference`] via [`Firestore::collection`] — a reference to
//!   a collection that can be queried and added to,
//! - a [`CollectionGroup`] via [`Firestore::collection_group`] — a query over
//!   every collection sharing the same ID.
//!
//! Documents are serialized and deserialized with [`serde`], so any type that
//! implements [`serde::Serialize`] can be written and any type that
//! implements [`serde::Deserialize`] can be read.
//!
//! # Examples
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use bouzuya_firestore_client::Firestore;
//! use bouzuya_firestore_client::FirestoreOptions;
//!
//! #[derive(serde::Deserialize, serde::Serialize)]
//! struct User {
//!     name: String,
//! }
//!
//! let firestore = Firestore::new(FirestoreOptions::default())?;
//!
//! // Write a document.
//! let document_reference = firestore.doc("users/alice")?;
//! document_reference
//!     .set(User {
//!         name: "Alice".to_owned(),
//!     })
//!     .await?;
//!
//! // Read it back.
//! let document_snapshot = document_reference.get().await?;
//! if let Some(user) = document_snapshot.data::<User>() {
//!     let user = user?;
//!     println!("{}", user.name);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Transactions
//!
//! Use [`Firestore::run_transaction`] to read and write atomically. The staged
//! writes are committed when the update function returns `Ok`, and rolled back
//! when it returns `Err`.
//!
//! # Connecting to the emulator
//!
//! Set the `FIRESTORE_EMULATOR_HOST` environment variable to connect to the
//! [Firestore emulator] instead of Google Cloud. See [`Firestore::new`] for
//! the full list of environment variables that affect client construction.
//!
//! [Cloud Firestore]: https://firebase.google.com/docs/firestore
//! [Node.js Firestore client]: https://docs.cloud.google.com/nodejs/docs/reference/firestore/latest/overview
//! [Firestore emulator]: https://firebase.google.com/docs/emulator-suite

mod collection_group;
mod collection_reference;
mod document_reference;
mod document_snapshot;
mod error;
mod field_path;
mod filter;
mod firestore;
mod firestore_client;
mod firestore_options;
mod into_field_path;
mod into_filter;
mod precondition;
mod private;
mod query;
mod query_document_snapshot;
mod query_snapshot;
mod timestamp;
mod transaction;
mod transaction_options;
mod write_result;

pub use self::collection_group::CollectionGroup;
pub use self::collection_reference::CollectionReference;
pub use self::document_reference::DocumentReference;
pub use self::document_snapshot::DocumentSnapshot;
pub use self::error::Error;
pub use self::field_path::FieldPath;
pub use self::filter::Filter;
pub use self::firestore::Firestore;
pub(crate) use self::firestore_client::FirestoreClient;
pub use self::firestore_options::FirestoreOptions;
pub use self::into_field_path::IntoFieldPath;
pub use self::into_filter::IntoFilter;
pub use self::precondition::Precondition;
pub use self::query::Query;
pub use self::query_document_snapshot::QueryDocumentSnapshot;
pub use self::query_snapshot::QuerySnapshot;
pub use self::timestamp::Timestamp;
pub use self::transaction::Transaction;
pub use self::transaction_options::TransactionOptions;
pub use self::write_result::WriteResult;
pub(crate) use serde_firestore_value::google;
