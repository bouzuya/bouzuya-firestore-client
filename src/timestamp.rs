pub struct Timestamp(prost_types::Timestamp);

impl Timestamp {
    pub(crate) fn new(timestamp: prost_types::Timestamp) -> Self {
        Self(timestamp)
    }

    pub(crate) fn to_prost_timestamp(&self) -> prost_types::Timestamp {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let prost_typestamp = prost_types::Timestamp {
            seconds: 1234567890,
            nanos: 123456789,
        };
        let timestamp = Timestamp::new(prost_typestamp.clone());
        assert_eq!(timestamp.to_prost_timestamp(), prost_typestamp);
    }

    #[test]
    fn test_to_prost_timestamp() {
        let prost_typestamp = prost_types::Timestamp {
            seconds: 1234567890,
            nanos: 123456789,
        };
        let timestamp = Timestamp::new(prost_typestamp.clone());
        assert_eq!(timestamp.to_prost_timestamp(), prost_typestamp);
    }
}
