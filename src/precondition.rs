use crate::Timestamp;

/// A condition that a document must satisfy for a write to be applied.
///
/// A `Precondition` is passed to write operations (such as
/// [`DocumentReference::delete`](crate::DocumentReference::delete)) to make
/// them conditional: the write is applied only if the target document
/// currently meets the condition, and otherwise fails. It carries at most
/// one condition — either an existence check ([`exists`](Self::exists)) or
/// an update-time check ([`last_update_time`](Self::last_update_time));
/// setting both is an error.
///
/// [`Precondition::default`] leaves both fields `None`, which imposes no
/// condition and applies the write unconditionally.
///
/// # Examples
///
/// ```
/// use bouzuya_firestore_client::Precondition;
///
/// // Require that the document already exists.
/// let _ = Precondition {
///     exists: Some(true),
///     ..Precondition::default()
/// };
///
/// // Apply the write unconditionally.
/// let _ = Precondition::default();
/// ```
#[derive(Default)]
pub struct Precondition {
    /// Requires the target document to exist, or not to exist.
    ///
    /// `Some(true)` makes the operation succeed only if the document
    /// currently exists; `Some(false)` only if it does not. `None` imposes
    /// no existence constraint.
    ///
    /// This field is mutually exclusive with
    /// [`last_update_time`](Self::last_update_time): setting both is an
    /// error.
    pub exists: Option<bool>,
    /// Requires the target document's last update time to match.
    ///
    /// `Some(timestamp)` makes the operation succeed only if the
    /// document's current update time equals `timestamp` — typically the
    /// [`update_time`](crate::DocumentSnapshot::update_time) of a snapshot
    /// read earlier, used to detect a concurrent modification. `None`
    /// imposes no update-time constraint.
    ///
    /// This field is mutually exclusive with [`exists`](Self::exists):
    /// setting both is an error.
    pub last_update_time: Option<Timestamp>,
}
