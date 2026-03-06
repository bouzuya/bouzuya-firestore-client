use crate::Timestamp;

pub struct WriteResult {
    write_time: Timestamp,
}

impl WriteResult {
    pub(crate) fn new(write_time: Timestamp) -> Self {
        Self { write_time }
    }

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
