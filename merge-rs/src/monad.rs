use crate::Merge;

/// Implementation of merge for the Option type. The merge is defined with the following
/// logic chart:
///
/// | Target  | Other   | Result           |
/// |---------|---------|------------------|
/// | None    | None    | None             |
/// | Some(a) | None    | Some(a)          |
/// | None    | Some(b) | Some(b)          |
/// | Some(a) | Some(b) | Some(a.merge(b)) |
///
/// Two Option instances can only merge if their containing data elements also support
/// merging.
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

/// Implementation of merge for the Result type. The merge strategy used is right-bias
/// towards the error case. If the target or right hand side is an [Err] instance, the
/// result will contain that [Err] value. When both values are [Ok], then the result will
/// be an [Ok] containing the result of merging the inner values.
impl<T: Clone + Merge, E: Clone + Merge> Merge for Result<T, E> {
    fn merge(&self, rhs: &Self) -> Self {
        match (self, rhs) {
            (Err(left), _) => Err(left.clone()),
            (_, Err(right)) => Err(right.clone()),
            (Ok(left), Ok(right)) => Ok(left.merge(right)),
        }
    }
}
