
pub trait Merge {
    fn merge(&self, other: &Self) -> Self;
}

pub trait MergeMut {
    fn merge_mut(&mut self, other: &Self);
}

pub mod mergeable;
mod monad;
mod collections;
