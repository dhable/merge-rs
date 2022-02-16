use std::vec::Vec;
use crate::{Merge, MergeMut};

impl<T: Clone> Merge for Vec<T> {
    fn merge(&self, rhs: &Self) -> Self {
        let mut res = self.clone();
        res.extend_from_slice(rhs);
        res
    }
}

impl<T: Clone> MergeMut for Vec<T> {
    fn merge_mut(&mut self, rhs: &Self) {
        self.extend_from_slice(rhs);
    }
}
