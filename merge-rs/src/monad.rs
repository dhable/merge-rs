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
            (Some(left), None) => Some(left.clone()),
            (None, Some(right)) => Some(right.clone()),
            (None, None) => None,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct MergeSpy(&'static str, bool);

    impl MergeSpy {
        fn new() -> Self {
            Self::with_label("spy")
        }

        fn with_label(label: &'static str) -> Self {
            MergeSpy(label, false)
        }

        fn is_merge_called(&self) -> bool {
            self.1
        }

        fn label(&self) -> &str {
            self.0
        }
    }

    impl Merge for MergeSpy {
        fn merge(&self, _: &Self) -> Self {
            MergeSpy("merged", true)
        }
    }

    #[test]
    fn test_option_merge_both_none() {
        let left: Option<MergeSpy> = None;
        let right: Option<MergeSpy> = None;

        let actual = left.merge(&right);
        assert!(actual.is_none());
    }

    #[test]
    fn test_option_merge_left_none() {
        let left = None;
        let right = Some(MergeSpy::new());

        let actual = left.merge(&right);
        assert!(matches!(actual, Some(res) if !res.is_merge_called()));
    }

    #[test]
    fn test_option_merge_right_none() {
        let left = Some(MergeSpy::new());
        let right = None;

        let actual = left.merge(&right);
        assert!(matches!(actual, Some(res) if !res.is_merge_called()));
    }

    #[test]
    fn test_option_merge_both_some() {
        let left = Some(MergeSpy::new());
        let right = Some(MergeSpy::new());

        let actual = left.merge(&right);
        assert!(matches!(actual, Some(res) if res.is_merge_called()));
    }

    #[test]
    fn test_result_merge_left_err() {
        let left = Err(MergeSpy::with_label("left"));
        let right = Ok(MergeSpy::with_label("right"));

        let actual = left.merge(&right);
        assert!(matches!(actual, Err(res) if res.label() == "left" && !res.is_merge_called()));
    }

    #[test]
    fn test_result_merge_left_both_err() {
        let left: Result<MergeSpy, MergeSpy> = Err(MergeSpy::with_label("left"));
        let right: Result<MergeSpy, MergeSpy> = Err(MergeSpy::with_label("right"));

        let actual = left.merge(&right);
        assert!(matches!(actual, Err(res) if res.label() == "left" && !res.is_merge_called()));
    }

    #[test]
    fn test_result_merge_right_err() {
        let left = Ok(MergeSpy::with_label("left"));
        let right = Err(MergeSpy::with_label("right"));

        let actual = left.merge(&right);
        assert!(matches!(actual, Err(res) if res.label() == "right" && !res.is_merge_called()));
    }

    #[test]
    fn test_result_merge_both_ok() {
        let left: Result<MergeSpy, MergeSpy> = Ok(MergeSpy::with_label("left"));
        let right: Result<MergeSpy, MergeSpy> = Ok(MergeSpy::with_label("right"));

        let actual = left.merge(&right);
        assert!(matches!(actual, Ok(res) if res.is_merge_called()));
    }
}
