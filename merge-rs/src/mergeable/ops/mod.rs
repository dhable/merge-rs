
macro_rules! impl_ops {
    ($x: tt, $y: tt) => {
        impl<T: $x <Output = T>> $x for Mergable<T> {
            type Output = Self;
            fn $y(self, rhs: Self) -> Self {
                Mergable::new(
                    self.inner.$y(rhs.inner),
                    self.strategy
                )
            }
        }

        impl<T: $x<Output = T>> $x <T> for Mergable<T> {
            type Output = Self;
            fn $y(self, rhs: T) -> Self {
                Mergable::new(self.inner.$y(rhs), self.strategy)
            }
        }
    };
}

mod arith;
mod bit;
