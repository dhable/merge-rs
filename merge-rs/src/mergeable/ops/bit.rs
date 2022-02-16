use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShlAssign, ShrAssign};
use crate::mergeable::Mergable;

impl_ops! {BitAnd, bitand}
impl_ops! {BitOr, bitor}
impl_ops! {BitXor, bitxor}
impl_ops! {Shl, shl}
impl_ops! {Shr, shr}

impl<T: Not<Output = T>> Not for Mergable<T> {
    type Output = Self;
    fn not(self) -> Self {
        Mergable::new(self.inner.not(), self.strategy)
    }
}

impl<T: BitAndAssign> BitAndAssign for Mergable<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner &= rhs.inner
    }
}

impl<T: BitAndAssign> BitAndAssign<T> for Mergable<T> {
    fn bitand_assign(&mut self, rhs: T) {
        self.inner &= rhs
    }
}

impl<T: BitOrAssign> BitOrAssign for Mergable<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner |= rhs.inner
    }
}

impl<T: BitOrAssign> BitOrAssign<T> for Mergable<T> {
    fn bitor_assign(&mut self, rhs: T) {
        self.inner |= rhs
    }
}

impl<T: BitXorAssign> BitXorAssign for Mergable<T> {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner ^= rhs.inner
    }
}

impl<T: BitXorAssign> BitXorAssign<T> for Mergable<T> {
    fn bitxor_assign(&mut self, rhs: T) {
        self.inner ^= rhs
    }
}

impl<T: ShlAssign> ShlAssign for Mergable<T> {
    fn shl_assign(&mut self, rhs: Self) {
        self.inner <<= rhs.inner
    }
}

impl<T: ShlAssign> ShlAssign<T> for Mergable<T> {
    fn shl_assign(&mut self, rhs: T) {
        self.inner <<= rhs
    }
}

impl<T: ShrAssign> ShrAssign for Mergable<T> {
    fn shr_assign(&mut self, rhs: Self) {
        self.inner >>= rhs.inner
    }
}

impl<T: ShrAssign> ShrAssign<T> for Mergable<T> {
    fn shr_assign(&mut self, rhs: T) {
        self.inner >>= rhs
    }
}
