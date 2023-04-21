# merge-rs

A small library that cuts down on the amount of code required to merge two arbitrary
structs into a new struct.

[Documentation](https://docs.rs/merge-rs)

```
[dependencies]
merge-rs = "0.3"
```

## Example

```rust
fn special_concat(left: &str, right: &String) -> Result<String, Box<Error>> {
    Ok(format!("{left}_{right}"))
}

#[derive(Debug, Merge)]
struct MyType {
    #[merge_field(skip)]
    transient_field: usize,
    #[merge_field(strategy = "special_concat")]
    label: String
}

fn main() {
    let first = MyType { transient_field: 123, label: "first".to_owned() };
    let second = MyType { transient_field: 456, label: "second".to_owned() };
    let merged = first.merge(&second).unwrap();
    println!("{merged:?}")
}
```

# Contributors

* Dan Hable <dan@danhable.com>
