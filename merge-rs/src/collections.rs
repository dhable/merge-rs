use crate::{Merge, MergeMut};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList};
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

/// Implementation of Merge for the HashMap type. The resulting HashMap is the union
/// of elements from the target HashMap and the right hand side HashMap. Where the same
/// key appears in both HashMaps, the resulting value is the value from target HashMap
/// merged with the value from the right hand side.
impl<K: Clone + Eq + Hash, V: Clone + Eq + Merge> Merge for HashMap<K, V> {
    fn merge(&self, rhs: &Self) -> Self {
        let mut res = self.clone();
        for (rhs_key, rhs_val) in rhs.iter() {
            let new_val = match res.get(rhs_key) {
                None => rhs_val.clone(),
                Some(existing_val) => existing_val.merge(rhs_val),
            };
            res.insert(rhs_key.clone(), new_val);
        }
        res
    }
}

impl<K: Clone + Eq + Hash, V: Clone + Eq + MergeMut> MergeMut for HashMap<K, V> {
    fn merge_mut(&mut self, other: &Self) {
        for (other_key, other_val) in other.iter() {
            self.entry(other_key.clone())
                .and_modify(|self_val| self_val.merge_mut(other_val))
                .or_insert_with(|| other_val.clone());
        }
    }
}

/// Implementation of Merge for the BTreeMap type. The resulting BTreeMap is the union
/// of elements from the target BTreeMap and the right hand side BTreeMap. Where the same
/// key appears in both BTreeMaps, the resulting value is the value from target BTreeMap
/// merged with the value from the right hand side.
impl<K: Clone + Eq + Hash + Ord, V: Clone + Eq + Merge> Merge for BTreeMap<K, V> {
    fn merge(&self, rhs: &Self) -> Self {
        let mut res = self.clone();
        for (rhs_key, rhs_val) in rhs.iter() {
            let new_val = match res.get(rhs_key) {
                None => rhs_val.clone(),
                Some(existing_val) => existing_val.merge(rhs_val),
            };
            res.insert(rhs_key.clone(), new_val);
        }
        res
    }
}


impl<K: Clone + Eq + Hash + Ord, V: Clone + Eq + MergeMut> MergeMut for BTreeMap<K, V> {
    fn merge_mut(&mut self, other: &Self) {
        for (other_key, other_val) in other.iter() {
            self.entry(other_key.clone())
                .and_modify(|self_val| self_val.merge_mut(other_val))
                .or_insert_with(|| other_val.clone());
        }
    }
}

/// Implementation of Merge for LinkedList type. The resulting LinkedList is a new list
/// with all the elements from the target list followed by all of the elements from the
/// right hand side list.
impl<T: Clone> Merge for LinkedList<T> {
    fn merge(&self, rhs: &Self) -> Self {
        let mut res = self.clone();
        res.merge_mut(rhs);
        res
    }
}

impl<T: Clone> MergeMut for LinkedList<T> {
    fn merge_mut(&mut self, rhs: &Self) {
        let mut rhs = rhs.clone();
        self.append(&mut rhs);
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

    impl Merge for usize {
        fn merge(&self, rhs: &Self) -> Self {
            self + rhs
        }
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
     * Vec Test Cases
     ****************************************************************************/
    merge_test!(
        linkedlist_both_values,
        LinkedList::from([1, 2, 3]),
        LinkedList::from([4, 5]),
        LinkedList::from([1, 2, 3, 4, 5])
    );
    merge_test!(
        linkedlist_target_empty,
        LinkedList::new(),
        LinkedList::from([4, 5]),
        LinkedList::from([4, 5])
    );
    merge_test!(
        linkedlist_rhs_empty,
        LinkedList::from([1, 2, 3]),
        LinkedList::new(),
        LinkedList::from([1, 2, 3])
    );
    merge_test!(
        linkedlist_both_empty,
        LinkedList::<usize>::new(),
        LinkedList::new(),
        LinkedList::new()
    );
    merge_test!(
        linkedlist_duplicates,
        LinkedList::from([1, 2, 3]),
        LinkedList::from([2, 3, 4]),
        LinkedList::from([1, 2, 3, 2, 3, 4])
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

    /****************************************************************************
     * HashMap Test Cases
     ****************************************************************************/
    merge_test!(
        hashmap_both_empty,
        HashMap::<String, usize>::new(),
        HashMap::new(),
        HashMap::new()
    );
    merge_test!(
        hashmap_lhs_empty,
        HashMap::from([("a", 1), ("b", 2)]),
        HashMap::new(),
        HashMap::from([("a", 1), ("b", 2)])
    );
    merge_test!(
        hashmap_rhs_empty,
        HashMap::new(),
        HashMap::from([("c", 1), ("d", 2)]),
        HashMap::from([("c", 1), ("d", 2)])
    );
    merge_test!(
        hashmap_no_overlapping_keys,
        HashMap::from([("a", 1), ("b", 2)]),
        HashMap::from([("c", 1), ("d", 2)]),
        HashMap::from([("a", 1), ("b", 2), ("c", 1), ("d", 2)])
    );
    merge_test!(
        hashmap_overlapping_keys,
        HashMap::from([("a", 1), ("b", 2), ("c", 3)]),
        HashMap::from([("c", 10), ("d", 20)]),
        HashMap::from([("a", 1), ("b", 2), ("c", 13), ("d", 20)])
    );

    /****************************************************************************
     * BTreeMap Test Cases
     ****************************************************************************/
    merge_test!(
        btreemap_both_empty,
        BTreeMap::<String, usize>::new(),
        BTreeMap::new(),
        BTreeMap::new()
    );
    merge_test!(
        btreemap_lhs_empty,
        BTreeMap::from([("a", 1), ("b", 2)]),
        BTreeMap::new(),
        BTreeMap::from([("a", 1), ("b", 2)])
    );
    merge_test!(
        btreemap_rhs_empty,
        BTreeMap::new(),
        BTreeMap::from([("c", 1), ("d", 2)]),
        BTreeMap::from([("c", 1), ("d", 2)])
    );
    merge_test!(
        btreemap_no_overlapping_keys,
        BTreeMap::from([("a", 1), ("b", 2)]),
        BTreeMap::from([("c", 1), ("d", 2)]),
        BTreeMap::from([("a", 1), ("b", 2), ("c", 1), ("d", 2)])
    );
    merge_test!(
        btreemap_overlapping_keys,
        BTreeMap::from([("a", 1), ("b", 2), ("c", 3)]),
        BTreeMap::from([("c", 10), ("d", 20)]),
        BTreeMap::from([("a", 1), ("b", 2), ("c", 13), ("d", 20)])
    );
}
