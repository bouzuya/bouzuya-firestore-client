use crate::Timestamp;

pub struct Precondition {
    pub exists: Option<bool>,
    pub last_update_time: Option<Timestamp>,
}
