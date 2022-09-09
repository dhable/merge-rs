use derivative::*;
use std::ops::*;
use crate::{Merge, MergeMut};

type Strategy<T> = fn(&T, &T) -> T;

/// Very often, types from the standard library and other crates will not implement the [Merge] or
/// [MergeMut] traits, preventing us from leveraging recursive merging with complex data structures.
/// Given that rust only allows trait implementations in the crate defining the trait or the struct,
/// this puts users in a tough position of writing more code.
/// 
/// The [Mergable] type is a type wrapper that implements the [Merge] and [MergeMut] traits using the
/// provided [Strategy] while exposing the underlying type through [Deref] and a number of operators
/// that forward the underlying implementation to the inner type.
/// 
/// ## Usage
/// ```
/// use merge_rs::Merge;
/// use merge_rs::mergeable::Mergable;
/// 
/// struct Message(String);
/// impl Message {
///   pub fn new(msg: String) -> Self {
///     Self(msg)
///   }
/// 
///   pub fn get(&self) -> &str {
///     &self.0
///   }
/// }
/// 
/// fn concat_messages(
///   left: &Message, 
///   right: &Message
/// ) -> Message {
///   Message::new(
///     format!("{} {}", left.get(), right.get())
///   )
/// }
/// 
/// let msg1 = Mergable::new(
///     Message::new("hello".to_string()),
///     concat_messages
/// );
/// let msg2 = Mergable::new(
///     Message::new("world".to_string()),
///     concat_messages
/// );
/// let greeting = msg1.merge(&msg2);
/// 
/// let hello_sring = msg1.get();
/// ```
#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub struct Mergable<T> {
    inner: T,
    #[derivative(Debug="ignore")]
    strategy: fn(&T, &T) -> T
}

impl<T> Mergable<T> {
    /// Construct a new Mergable type with a given inner type and strategy for
    /// merging instances of the inner types together.
    pub fn new(inner: T, strategy: Strategy<T>) -> Self {
        Self { inner, strategy }
    }
}

impl<T> Merge for Mergable<T> {
    fn merge(&self, other: &Self) -> Self {
        Mergable::new(
            (self.strategy)(&self.inner, &other.inner), 
            self.strategy
        )
    }
}

impl<T> MergeMut for Mergable<T> {
    fn merge_mut(&mut self, other: &Self) {
        self.inner = (self.strategy)(&self.inner, &other.inner);
    }
}

impl<T> Deref for Mergable<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Mergable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

mod ops;
