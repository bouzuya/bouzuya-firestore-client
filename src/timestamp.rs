#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Timestamp(prost_types::Timestamp);

impl Timestamp {
    pub(crate) fn from_prost_timestamp(timestamp: prost_types::Timestamp) -> Self {
        Self(timestamp)
    }

    pub(crate) fn into_prost_timestamp(self) -> prost_types::Timestamp {
        self.0
    }
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .seconds
            .cmp(&other.0.seconds)
            .then_with(|| self.0.nanos.cmp(&other.0.nanos))
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
