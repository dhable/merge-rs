use std::ops::{Add, Div, Mul, Neg, Rem, Sub, AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use crate::mergeable::Mergable;

impl_ops! {Add, add}
impl_ops! {Div, div}
impl_ops! {Mul, mul}
impl_ops! {Rem, rem}
impl_ops! {Sub, sub}

impl<T> Neg for Mergable<T>
where
    T: Neg<Output = T>
{
    type Output = Self;
    fn neg(self) -> Self {
        Mergable::new(self.inner.neg(), self.strategy)
    }
}

impl<T> AddAssign for Mergable<T>
where
    T: AddAssign
{
    fn add_assign(&mut self, rhs: Self) {
        self.inner += rhs.inner
    }
}

impl<T> AddAssign<T> for Mergable<T> 
where
    T: AddAssign
{
    fn add_assign(&mut self, rhs: T) {
        self.inner += rhs
    }
}

impl<T> DivAssign for Mergable<T> 
where
    T: DivAssign
{
    fn div_assign(&mut self, rhs: Self) {
        self.inner /= rhs.inner
    }
}

impl<T> DivAssign<T> for Mergable<T> 
where
    T: DivAssign
{
    fn div_assign(&mut self, rhs: T) {
        self.inner /= rhs
    }
}

impl<T> MulAssign for Mergable<T>
where 
    T: MulAssign
{
    fn mul_assign(&mut self, rhs: Self) {
        self.inner *= rhs.inner
    }
}

impl<T> MulAssign<T> for Mergable<T> 
where
    T: MulAssign
{
    fn mul_assign(&mut self, rhs: T) {
        self.inner *= rhs
    }
}

impl<T> RemAssign for Mergable<T> 
where
    T: RemAssign
{
    fn rem_assign(&mut self, rhs: Self) {
        self.inner %= rhs.inner
    }
}

impl<T> RemAssign<T> for Mergable<T> 
where
    T: RemAssign
{
    fn rem_assign(&mut self, rhs: T) {
        self.inner %= rhs
    }
}

impl<T> SubAssign for Mergable<T>
where
    T: SubAssign
{
    fn sub_assign(&mut self, rhs: Self) {
        self.inner -= rhs.inner
    }
}

impl<T> SubAssign<T> for Mergable<T> 
where
    T: SubAssign
{
    fn sub_assign(&mut self, rhs: T) {
        self.inner -= rhs
    }
}
