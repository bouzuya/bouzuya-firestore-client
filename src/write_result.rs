#[cfg(doc)]
use crate::DocumentReference;
use crate::Timestamp;

/// The result of a single write operation.
///
/// A `WriteResult` wraps the write time set by the Firestore servers on a
/// successful write. Obtain one from [`DocumentReference::create`],
/// [`DocumentReference::set`], [`DocumentReference::update`], or
/// [`DocumentReference::delete`]; read the server-side commit time via
/// [`write_time`](Self::write_time).
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use bouzuya_firestore_client::Firestore;
/// use bouzuya_firestore_client::FirestoreOptions;
/// use std::collections::HashMap;
///
/// let firestore = Firestore::new(FirestoreOptions::default())?;
/// let document_reference = firestore.doc("rooms/roomA")?;
/// let write_result = document_reference
///     .create(HashMap::<String, String>::new())
///     .await?;
/// let _write_time = write_result.write_time();
/// # Ok(())
/// # }
/// ```
pub struct WriteResult {
    write_time: Timestamp,
}

impl WriteResult {
    pub(crate) fn new(write_time: Timestamp) -> Self {
        Self { write_time }
    }
}

impl WriteResult {
    /// Returns the write time as set by the Firestore servers.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use bouzuya_firestore_client::Firestore;
    /// use bouzuya_firestore_client::FirestoreOptions;
    /// use std::collections::HashMap;
    ///
    /// let firestore = Firestore::new(FirestoreOptions::default())?;
    /// let document_reference = firestore.doc("rooms/roomA")?;
    /// let write_result = document_reference
    ///     .create(HashMap::<String, String>::new())
    ///     .await?;
    /// let _write_time = write_result.write_time();
    /// # Ok(())
    /// # }
    /// ```
    pub fn write_time(&self) -> Timestamp {
        self.write_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let write_time = Timestamp::from_prost_timestamp(prost_types::Timestamp {
            seconds: 1234567890,
            nanos: 123456789,
        });
        let write_result = WriteResult::new(write_time);
        assert_eq!(write_result.write_time(), write_time);
    }
}
