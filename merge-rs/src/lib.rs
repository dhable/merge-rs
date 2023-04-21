/// Trait for types that support merging two values and producing a new value
/// with the result of the merge. The input values are left unchanged.
pub trait Merge {
    #[must_use = "Did you want to implement MergeMut instead?"]
    fn merge(&self, other: &Self) -> Self;
}

/// Trait for types that support merging two values and replace the target values
/// with the result of the merge operation.
pub trait MergeMut {
    fn merge_mut(&mut self, other: &Self);
}

mod collections;
pub mod mergeable;
mod monad;
