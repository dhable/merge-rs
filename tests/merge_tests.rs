use merge_rs::{Merge};
use merge_derive::{Merge};

fn concat_string(lhs: &str, rhs: &String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(lhs.to_owned() + rhs)
}

fn sum<T: std::ops::Add<Output=T> + Copy>(lhs: &T, rhs: &T) -> Result<T, Box<dyn std::error::Error>> {
    Ok(*lhs + *rhs)
}

fn max<T: PartialOrd + Copy>(lhs: &T, rhs: &T) -> Result<T, Box<dyn std::error::Error>> {
    if *lhs < *rhs {
        Ok(*rhs)
    } else {
        Ok(*lhs)
    }
}

#[derive(Debug, Merge)]
struct NestedStruct {
    #[merge_field(strategy = "concat_string")]
    name: String,
    #[merge_field(skip)]
    name_len: usize,
    #[merge_field(strategy = "sum")]
    age: u8,
}

/// Test Case 1 - Named Structs
#[derive(Debug, Merge)]
struct Named {
    #[merge_field(skip)]
    id: u64,
    #[merge_field(strategy = "max")]
    timestamp: u64,
    person: NestedStruct,
}

#[test]
fn test_named_structs() {
    let left = Named {
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
    let actual = left.merge(&right).expect("merge should be successful");
    assert_eq!(actual.id, 123);
    assert_eq!(actual.timestamp, 1700265910060);
    assert_eq!(actual.person.name, "ABB".to_string());
    assert_eq!(actual.person.name_len, 1);
    assert_eq!(actual.person.age, 34);
}

/// Test Case 2 - Unit Structs
#[derive(Debug, Merge)]
struct UnitStruct;

#[test]
fn test_unit_structs() {
    let left = UnitStruct;
    let right = UnitStruct;
    let _actual = left.merge(&right).expect("merge should be successful");
}

/// Test Case 3 - Tuple Structs
#[derive(Debug, Merge)]
struct TupleStruct(
    #[merge_field(skip)] u64,
    #[merge_field(strategy = "max")] u64,
    NestedStruct,
);

#[test]
fn test_tuple_struct() {
    let left = TupleStruct(789, 1700319581526, NestedStruct {
        name: "C".to_string(),
        name_len: 1,
        age: 5,
    });
    let right = TupleStruct(987, 1700319612124, NestedStruct {
        name: "DD".to_string(),
        name_len: 2,
        age: 65,
    });
    let actual = left.merge(&right).expect("merge should be successful");
    assert_eq!(actual.0, 789);
    assert_eq!(actual.1, 1700319612124);
    assert_eq!(actual.2.name, "CDD".to_string());
    assert_eq!(actual.2.name_len, 1);
    assert_eq!(actual.2.age, 70);
}
