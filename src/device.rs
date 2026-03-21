#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Device {
    id: String,
    name: String,
}

impl Device {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}

impl Device {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
