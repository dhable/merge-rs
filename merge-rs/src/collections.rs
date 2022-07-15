use crate::{Merge, MergeMut};
use std::collections::{BTreeSet, HashSet};
use std::hash::Hash;
use std::vec::Vec;

/// Implementation of Merge for the Vec type. The resulting Vec contains all of the elements
/// from the target Vec followed by all of the elements from the right hand side Vec.
impl<T: Clone> Merge for Vec<T> {
    fn merge(&self, rhs: &Self) -> Self {
        let mut res = self.clone();
        res.merge_mut(rhs);
        res
    }
}

impl<T: Clone> MergeMut for Vec<T> {
    fn merge_mut(&mut self, rhs: &Self) {
        self.extend_from_slice(rhs);
    }
}

///Implementation of Merge for the HashSet type. The resulting HashSet is the union of
/// elements from the target HashSet and the right hand side HashSet. Order of elements in
/// the merged result may differ based on the hashing algorithm.
impl<T: Clone + Eq + Hash> Merge for HashSet<T> {
    fn merge(&self, rhs: &Self) -> Self {
        let mut res: HashSet<T> = self.clone();
        res.merge_mut(rhs);
        res
    }
}

impl<T: Clone + Eq + Hash> MergeMut for HashSet<T> {
    fn merge_mut(&mut self, rhs: &Self) {
        for item in rhs {
            self.insert(item.clone());
        }
    }
}

/// Implementation of Merge for the BTreeSet type. The resulting BTreeSet is the union of
/// elements from the target BTreeSet and the right hand side BTreeSet. Elements in the resulting
/// BTreeSet will be their correct order as defined in the BTreeSet type.
impl<T: Clone + Eq + Ord> Merge for BTreeSet<T> {
    fn merge(&self, rhs: &Self) -> Self {
        let mut res = self.clone();
        res.merge_mut(rhs);
        res
    }
}

impl<T: Clone + Eq + Ord> MergeMut for BTreeSet<T> {
    fn merge_mut(&mut self, rhs: &Self) {
        for item in rhs {
            self.insert(item.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use concat_idents::concat_idents;

    macro_rules! merge_test {
        ($name:ident, $target:expr, $rhs:expr, $expected:expr) => {
            concat_idents!(fn_name = $name, "_merge_test" {
                #[test]
                fn fn_name() {
                    let actual = $target.merge(&$rhs);
                    assert_eq!(actual, $expected);
                }
            });
        };
    }

    /****************************************************************************
     * Vec Test Cases
     ****************************************************************************/
    merge_test!(
        vecs_both_values,
        vec![1, 2, 3],
        vec![4, 5],
        vec![1, 2, 3, 4, 5]
    );
    merge_test!(vecs_target_empty, Vec::new(), vec![4, 5], vec![4, 5]);
    merge_test!(vecs_rhs_empty, vec![1, 2, 3], Vec::new(), vec![1, 2, 3]);
    merge_test!(vecs_both_empty, Vec::<usize>::new(), Vec::new(), Vec::new());
    merge_test!(
        vecs_duplicates,
        vec![1, 2, 3, 4],
        vec![3, 4, 5, 6],
        vec![1, 2, 3, 4, 3, 4, 5, 6]
    );

    /****************************************************************************
     * HashSet Test Cases
     ****************************************************************************/
    merge_test!(
        hashset_both_values,
        HashSet::from([1, 2, 3]),
        HashSet::from([4, 5]),
        HashSet::from([1, 2, 3, 4, 5])
    );
    merge_test!(
        hashset_target_empty,
        HashSet::new(),
        HashSet::from([4, 5]),
        HashSet::from([4, 5])
    );
    merge_test!(
        hashset_rhs_empty,
        HashSet::from([1, 2, 3]),
        HashSet::new(),
        HashSet::from([1, 2, 3])
    );
    merge_test!(
        hashset_both_empty,
        HashSet::<usize>::new(),
        HashSet::new(),
        HashSet::new()
    );
    merge_test!(
        hashset_dups,
        HashSet::from([1, 2, 3]),
        HashSet::from([3, 4, 5]),
        HashSet::from([1, 2, 3, 4, 5])
    );

    /****************************************************************************
     * BTreeSet Test Cases
     ****************************************************************************/
    merge_test!(
        btreeset_both_values,
        BTreeSet::from([1, 2, 3]),
        BTreeSet::from([4, 5]),
        BTreeSet::from([1, 2, 3, 4, 5])
    );
    merge_test!(
        btreeset_target_empty,
        BTreeSet::new(),
        BTreeSet::from([4, 5]),
        BTreeSet::from([4, 5])
    );
    merge_test!(
        btreeset_rhs_empty,
        BTreeSet::from([1, 2, 3]),
        BTreeSet::new(),
        BTreeSet::from([1, 2, 3])
    );
    merge_test!(
        btreeset_both_empty,
        BTreeSet::<usize>::new(),
        BTreeSet::new(),
        BTreeSet::new()
    );
    merge_test!(
        btreeset_dups,
        BTreeSet::from([1, 2, 3]),
        BTreeSet::from([3, 4, 5]),
        BTreeSet::from([1, 2, 3, 4, 5])
    );
}
