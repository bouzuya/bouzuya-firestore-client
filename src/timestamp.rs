use crate::Error;

/// 0001-01-01T00:00:00Z as seconds since UNIX epoch.
const MIN_SECONDS: i64 = -62_135_596_800;
/// 9999-12-31T23:59:59Z as seconds since UNIX epoch.
const MAX_SECONDS: i64 = 253_402_300_799;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Timestamp(prost_types::Timestamp);

impl Timestamp {
    /// Creates a new [`Timestamp`] from the given number of milliseconds.
    ///
    /// `millis` is the number of milliseconds since the Unix epoch
    /// 1970-01-01T00:00:00Z.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] when `millis` falls outside the supported range,
    /// i.e. before 0001-01-01T00:00:00Z or after 9999-12-31T23:59:59.999Z.
    ///
    /// # Examples
    ///
    /// ```
    /// use bouzuya_firestore_client::Timestamp;
    ///
    /// let timestamp = Timestamp::from_millis(1_500)?;
    /// assert_eq!(timestamp.seconds(), 1);
    /// assert_eq!(timestamp.nanoseconds(), 500_000_000);
    /// # Ok::<(), bouzuya_firestore_client::Error>(())
    /// ```
    pub fn from_millis(millis: i64) -> Result<Self, Error> {
        let seconds = millis.div_euclid(1_000);
        let nanos = (millis.rem_euclid(1_000) * 1_000_000) as i32;
        Self::new(seconds, nanos)
    }

    pub(crate) fn from_prost_timestamp(timestamp: prost_types::Timestamp) -> Self {
        Self(timestamp)
    }

    /// Creates a new [`Timestamp`].
    ///
    /// `seconds` is the number of seconds of UTC time since the Unix epoch
    /// 1970-01-01T00:00:00Z, and must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    ///
    /// `nanoseconds` is the non-negative fractions of a second at nanosecond
    /// resolution, and must be from 0 to 999,999,999 inclusive. Negative
    /// `seconds` values with fractions must still have non-negative
    /// `nanoseconds` values that count forward in time.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] when `seconds` or `nanoseconds` falls outside
    /// the ranges above.
    ///
    /// # Examples
    ///
    /// ```
    /// use bouzuya_firestore_client::Timestamp;
    ///
    /// let timestamp = Timestamp::new(1, 500_000_000)?;
    /// assert_eq!(timestamp.seconds(), 1);
    /// assert_eq!(timestamp.nanoseconds(), 500_000_000);
    /// # Ok::<(), bouzuya_firestore_client::Error>(())
    /// ```
    pub fn new(seconds: i64, nanoseconds: i32) -> Result<Self, Error> {
        if !(MIN_SECONDS..=MAX_SECONDS).contains(&seconds) {
            return Err(Error::custom(format!("seconds out of range: {}", seconds)));
        }
        if !(0..=999_999_999).contains(&nanoseconds) {
            return Err(Error::custom(format!(
                "nanoseconds out of range: {}",
                nanoseconds
            )));
        }
        Ok(Self(prost_types::Timestamp {
            seconds,
            nanos: nanoseconds,
        }))
    }

    /// Creates a new [`Timestamp`] with the current date, with millisecond
    /// precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use bouzuya_firestore_client::Timestamp;
    ///
    /// let timestamp = Timestamp::now();
    /// assert_eq!(timestamp.nanoseconds() % 1_000_000, 0);
    /// ```
    pub fn now() -> Self {
        Self::from_millis(
            i64::try_from(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("SystemTime before UNIX epoch")
                    .as_millis(),
            )
            .expect("millis overflow"),
        )
        .expect("now is in valid range")
    }
}

impl Timestamp {
    /// Returns the non-negative fractions of a second at nanosecond
    /// resolution.
    ///
    /// The value is in the range 0 to 999,999,999 inclusive. Even when
    /// [`seconds`](Self::seconds) is negative, the returned value remains
    /// non-negative and counts forward in time.
    ///
    /// # Examples
    ///
    /// ```
    /// use bouzuya_firestore_client::Timestamp;
    ///
    /// assert_eq!(Timestamp::new(1, 500_000_000)?.nanoseconds(), 500_000_000);
    /// assert_eq!(Timestamp::from_millis(-1_500)?.nanoseconds(), 500_000_000);
    /// # Ok::<(), bouzuya_firestore_client::Error>(())
    /// ```
    pub fn nanoseconds(&self) -> i32 {
        self.0.nanos
    }

    /// Returns the number of seconds of UTC time since the Unix epoch
    /// 1970-01-01T00:00:00Z.
    ///
    /// The value is in the range corresponding to 0001-01-01T00:00:00Z
    /// through 9999-12-31T23:59:59Z inclusive.
    ///
    /// # Examples
    ///
    /// ```
    /// use bouzuya_firestore_client::Timestamp;
    ///
    /// assert_eq!(Timestamp::new(1, 500_000_000)?.seconds(), 1);
    /// assert_eq!(Timestamp::from_millis(-1_500)?.seconds(), -2);
    /// # Ok::<(), bouzuya_firestore_client::Error>(())
    /// ```
    pub fn seconds(&self) -> i64 {
        self.0.seconds
    }

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
        let timestamp = Timestamp::from_prost_timestamp(prost_typestamp);
        assert_eq!(timestamp.into_prost_timestamp(), prost_typestamp);
    }

    #[test]
    fn test_into_prost_timestamp() {
        let prost_typestamp = prost_types::Timestamp {
            seconds: 1234567890,
            nanos: 123456789,
        };
        let timestamp = Timestamp::from_prost_timestamp(prost_typestamp);
        assert_eq!(timestamp.into_prost_timestamp(), prost_typestamp);
    }
}
