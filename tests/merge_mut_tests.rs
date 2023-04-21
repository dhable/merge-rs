use merge_rs::{MergeMut};
use merge_derive::{MergeMut};

fn concat_string(lhs: &mut String, rhs: &str) {
    lhs.push_str(rhs);
}

fn sum<T: std::ops::AddAssign + Copy>(lhs: &mut T, rhs: &T) {
    *lhs += *rhs;
}

fn max<T: PartialOrd + Copy>(lhs: &mut T, rhs: &T) {
    if *lhs < *rhs {
        *lhs = *rhs;
    }
}

#[derive(Debug, MergeMut)]
struct NestedStruct {
    #[merge_field(strategy = "concat_string")]
    name: String,
    #[merge_field(skip)]
    name_len: usize,
    #[merge_field(strategy = "sum")]
    age: u8,
}

/// Test Case 1 - Named Structs
#[derive(Debug, MergeMut)]
struct Named {
    #[merge_field(skip)]
    id: u64,
    #[merge_field(strategy = "max")]
    timestamp: u64,
    person: NestedStruct,
}

#[test]
fn test_named_structs() {
    let mut left = Named {
        id: 123,
        timestamp: 1700265850314,
        person: NestedStruct {
            name: "A".to_string(),
            name_len: 1,
            age: 22,
        },
    };
    let right = Named {
        id: 456,
        timestamp: 1700265910060,
        person: NestedStruct {
            name: "BB".to_string(),
            name_len: 2,
            age: 12,
        },
    };
    left.merge_mut(&right).expect("merge should be successful");
    assert_eq!(left.id, 123);
    assert_eq!(left.timestamp, 1700265910060);
    assert_eq!(left.person.name, "ABB".to_string());
    assert_eq!(left.person.name_len, 1);
    assert_eq!(left.person.age, 34);
}

/// Test Case 2 - Unit Structs
#[derive(Debug, MergeMut)]
struct UnitStruct;

#[test]
fn test_unit_structs() {
    let mut left = UnitStruct;
    let right = UnitStruct;
    left.merge_mut(&right).expect("merge should be successful");
}

/// Test Case 3 - Tuple Structs
#[derive(Debug, MergeMut)]
struct TupleStruct(
    #[merge_field(skip)] u64,
    #[merge_field(strategy = "max")] u64,
    NestedStruct,
);

#[test]
fn test_tuple_struct() {
    let mut left = TupleStruct(789, 1700319581526, NestedStruct {
        name: "C".to_string(),
        name_len: 1,
        age: 5,
    });
    let right = TupleStruct(987, 1700319612124, NestedStruct {
        name: "DD".to_string(),
        name_len: 2,
        age: 65,
    });
    left.merge_mut(&right).expect("merge should be successful");
    assert_eq!(left.0, 789);
    assert_eq!(left.1, 1700319612124);
    assert_eq!(left.2.name, "CDD".to_string());
    assert_eq!(left.2.name_len, 1);
    assert_eq!(left.2.age, 70);
}
