/// Trait for types that support merging two values and producing a new value
/// with the result of the merge. The input values are left unchanged.
pub trait Merge {
    fn merge(&self, other: &Self) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;
}

/// Trait for types that support merging two values and replace the target values
/// with the result of the merge operation.
pub trait MergeMut {
    fn merge_mut(&mut self, other: &Self) -> Result<(), Box<dyn std::error::Error>>;
}
