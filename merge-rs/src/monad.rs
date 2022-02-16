use crate::Merge;

impl<T: Clone + Merge> Merge for Option<T> {
    fn merge(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Some(left), Some(right)) => Some(left.merge(right)),
            (Some(left), _) => Some(left.clone()),
            (_, Some(right)) => Some(right.clone()),
            _ => None,
        }
    }
}

impl<T: Clone + Merge, E: Clone + Merge> Merge for Result<T, E> {
    fn merge(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Err(left), _) => Err(left.clone()),
            (_, Err(right)) => Err(right.clone()),
            (Ok(left), Ok(right)) => Ok(left.merge(right))
        }
    }
}
