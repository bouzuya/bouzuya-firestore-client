use crate::QueryDocumentSnapshot;

#[derive(Clone)]
pub struct QuerySnapshot {
    docs: Vec<QueryDocumentSnapshot>,
}

impl QuerySnapshot {
    #[allow(dead_code)]
    pub(crate) fn new(docs: Vec<QueryDocumentSnapshot>) -> Self {
        Self { docs }
    }
}

impl QuerySnapshot {
    pub fn docs(&self) -> Vec<QueryDocumentSnapshot> {
        self.docs.clone()
    }

    pub fn empty(&self) -> bool {
        self.docs.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::QuerySnapshot;

    #[test]
    fn test_new() {
        let _qs = QuerySnapshot::new(vec![]);
    }
}
