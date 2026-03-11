use crate::google;

pub struct Transaction {
    pub(crate) transaction: Vec<u8>,
    pub(crate) writes: Vec<google::firestore::v1::Write>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction() {
        let transaction = Transaction {
            transaction: vec![1, 2, 3],
            writes: vec![],
        };
        assert_eq!(transaction.transaction, vec![1, 2, 3]);
        assert_eq!(transaction.writes.len(), 0);
    }

    #[test]
    fn test_writes() {
        let transaction = Transaction {
            transaction: vec![],
            writes: vec![google::firestore::v1::Write::default()],
        };
        assert_eq!(transaction.writes.len(), 1);
    }
}
