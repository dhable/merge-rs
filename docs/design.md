# merge-rs Design

`merge-rs` was designed to support a customizable, recursive hook for merging
complex structs into a new version based on rules defined in code. The API and
desired behaviors are largely influenced by the [Semigroup type class][2] from the
[Scala cats library][1].

## Problem

Rust's struct type allows us to model complex data structures that consists of
multiple fields with different data types. Representing multiple fields as a single
unit is often useful when dealing with database records, configuration options or
any number of domain specific concepts. As a code base grows, we may introduce new
versions of these objects and need to combine multiple versions into a single
instance.

For example, an application may need to read configuration options from the command
line, environment variables, a legacy configuration file and a new configuration file.
One would naturally like to have these different instances of configuration merged
into a single representation with a minimal amount of code. Those configurations
could either be modeled as `Hashmap` instances at the loss of compile time type
checks or we could be left to write a large amount of boiler plate code.

## Goals

When setting out to design `merge-rs`, the library had to satisfy a few constraints
largely driven by the use case of configuration merging.

### Prefer immutable usage as a default

Driven by past experience with highly concurrent, async code bases, immutable code
is often preferred as it is easier to reason about. There is also a number of places
in rust code is given immutable references to data and while it's possible to obtain
a mutable reference, it's often ugly and increases the developer's cognitive load.

### Allow mutable usage for performance

Even though the design supports immutable usage, it should also provide a way for
an implantation to provide an optimized, mutable version to support use cases where
memory copies are expensive or prohibited. This may be the case in hot code paths or
in embedded applications.

### Provide sane default implementations

As inspired by the [Scala cats library][1], there are default implementations that
can be provided for built-in data types, like `Option<T>`, that would be tedious for
a user to provide every time `merge-rs` is added to a project. Rust's restrictions
on when a type can also implement a trait also make being able to provide sane
defaults even more important.

### Support user defined strategies

Even with default implementations, there are going to be times when a specific field,
even of the same type, might need a different strategy. It should be possible without
too much effort to support such a case.

## Solution

The heart of `merge-rs` consists is a pair of traits with a single method each - `Merge`
and `MergeMut`. By implementing one or both traits on all the necessary types in a
code base would consist of a lot of boilerplate code but be sufficient to allow types
to merge into new objects.

The `merge` and `merge_mut` method are kept into discrete traits to support the first
two goals. We encourage writing immutable code as much as possible for higher level
applications but provide a means to support lower level, mutable implementations when
needed.

Derive macros have been added to the crate to reduce the boilerplate code that needs
to be written. Beyond just calling `merge` or `merge_mut` on each field, the derive
macro allows skipping fields or defining a function to use for merging. This provides
flexibility that allows `merge-rs` to be used on types where it is not possible to provide
a trait implementation - e.g. externally defined types.

Two common cases, `Result` and `Option`, do have standard implementations for `Merge`.
The behavior of these types is modeled after the [Scala cats][1] implementation. Users
who wish to have a different behavior can always define a merge strategy function instead.

## Inspiration and Prior Art

The solution was driven by experience with the [rust `merge` crate][3]. It was the
starting point for solving the problem of merging multiple configurations but lacked
a flexible enough design to support some particular requirements around default
values.

The idea of a type class wrapper was driven largely through experience with [Scala cats][1]
and writing functional Scala code. Rust does provide some wrinkles, it largely supports
a similar design.


[1]: https://typelevel.org/cats

[2]: https://typelevel.org/cats/typeclasses/semigroup.html

[3]: https://crates.io/crates/merge
