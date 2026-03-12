use crate::Timestamp;

#[derive(Default)]
pub struct Precondition {
    pub exists: Option<bool>,
    pub last_update_time: Option<Timestamp>,
}
