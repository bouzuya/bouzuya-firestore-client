use crate::Timestamp;

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
    pub last_update_time: Option<Timestamp>,
}
