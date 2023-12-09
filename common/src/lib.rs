use std::ops::Deref;

pub trait StrExt {
    fn to_i32(&self) -> i32;
}

impl<T> StrExt for T
where
    T: Deref<Target = str>,
{
    fn to_i32(&self) -> i32 {
        self.parse::<i32>().expect("valid i32")
    }
}

pub trait Gcd {
    fn gcd(lhs: Self, rhs: Self) -> Self;
    fn lcm(lhs: Self, rhs: Self) -> Self;
}

macro_rules! gcd_unsigned_impl {
    ($ty:ty) => {
        impl Gcd for $ty {
            fn gcd(mut u: Self, mut v: Self) -> Self {
                if u == 0 {
                    return v;
                }
                if v == 0 {
                    return u;
                }

                let min_shift = (u | v).trailing_zeros();
                u >>= u.trailing_zeros();

                while v != 0 {
                    v >>= v.trailing_zeros();

                    if u > v {
                        std::mem::swap(&mut u, &mut v);
                    }
                    v -= u;
                }
                u << min_shift
            }
            #[inline]
            fn lcm(lhs: Self, rhs: Self) -> Self {
                (lhs * rhs) / Self::gcd(lhs, rhs)
            }
        }
    };
}

gcd_unsigned_impl!(usize);
