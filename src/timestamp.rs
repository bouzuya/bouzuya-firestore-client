#[derive(Clone)]
pub struct Timestamp(prost_types::Timestamp);

impl Timestamp {
    pub(crate) fn from_prost_timestamp(timestamp: prost_types::Timestamp) -> Self {
        Self(timestamp)
    }

    pub(crate) fn into_prost_timestamp(self) -> prost_types::Timestamp {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_prost_timestamp() {
        let prost_typestamp = prost_types::Timestamp {
            seconds: 1234567890,
            nanos: 123456789,
        };
        let timestamp = Timestamp::from_prost_timestamp(prost_typestamp.clone());
        assert_eq!(timestamp.into_prost_timestamp(), prost_typestamp);
    }

    #[test]
    fn test_into_prost_timestamp() {
        let prost_typestamp = prost_types::Timestamp {
            seconds: 1234567890,
            nanos: 123456789,
        };
        let timestamp = Timestamp::from_prost_timestamp(prost_typestamp.clone());
        assert_eq!(timestamp.into_prost_timestamp(), prost_typestamp);
    }
}
