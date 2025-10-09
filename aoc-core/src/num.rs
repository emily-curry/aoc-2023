use std::iter::{empty, Product, Sum};
use std::ops::{Div, Mul, Rem};

pub trait Zero<T> {
    fn zero() -> Self;
}

impl<T> Zero<T> for T
where
    T: Sum,
{
    fn zero() -> Self {
        empty().sum()
    }
}

pub trait One<T> {
    fn one() -> Self;
}

impl<T> One<T> for T
where
    T: Product,
{
    fn one() -> Self {
        empty().product()
    }
}

pub trait Bounded {
    const MIN: Self;
    const MAX: Self;
}

macro_rules! impl_bounded {
    ( $( $x:ty ),* ) => {

            $(
            impl Bounded for $x {
                const MIN: Self = <$x>::MIN;
                const MAX: Self = <$x>::MAX;
            }
        )*

    };
}

impl_bounded!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128);

pub trait GreatestCommonFactor<T> {
    fn gcf(&self, other: &T) -> T;
}

impl<T> GreatestCommonFactor<T> for T
where
    T: PartialEq + PartialOrd + Copy + Sum + Rem<Output = T>,
{
    fn gcf(&self, other: &T) -> T {
        if self == other {
            return *self;
        }
        let mut a = *self;
        let mut b = *other;
        if b > a {
            let temp = a;
            a = b;
            b = temp;
        }
        while b > T::zero() {
            let temp = a;
            a = b;
            b = temp % b;
        }
        a
    }
}

pub trait LeastCommonMultiple<T> {
    fn lcm(&self, other: &T) -> T;
}

impl<T> LeastCommonMultiple<T> for T
where
    T: PartialEq + PartialOrd + Copy + Sum + Rem<Output = T> + Div<Output = T> + Mul<Output = T>,
{
    fn lcm(&self, other: &T) -> T {
        *self * (*other / self.gcf(other))
    }
}
