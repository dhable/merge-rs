use derivative::*;
use std::ops::*;
use crate::{Merge, MergeMut};

type Strategy<T> = fn(&T, &T) -> T;

#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub struct Mergable<T> {
    inner: T,
    #[derivative(Debug="ignore")]
    strategy: fn(&T, &T) -> T
}

impl<T> Mergable<T> {
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
