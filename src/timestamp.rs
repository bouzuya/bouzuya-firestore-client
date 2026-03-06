#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Timestamp(prost_types::Timestamp);

impl Timestamp {
    pub fn from_millis(millis: i64) -> Self {
        Self(prost_types::Timestamp {
            seconds: millis.div_euclid(1_000),
            nanos: (millis.rem_euclid(1_000) * 1_000_000) as i32,
        })
    }

    pub(crate) fn from_prost_timestamp(timestamp: prost_types::Timestamp) -> Self {
        Self(timestamp)
    }
}

impl Timestamp {
    pub fn to_millis(&self) -> i64 {
        (self.0.seconds * 1_000) + (i64::from(self.0.nanos) / 1_000_000)
    }

    pub(crate) fn into_prost_timestamp(self) -> prost_types::Timestamp {
        self.0
    }
}

impl std::fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Timestamp")
            .field("seconds", &self.0.seconds)
            .field("nanos", &self.0.nanos)
            .finish()
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
