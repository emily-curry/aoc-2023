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
