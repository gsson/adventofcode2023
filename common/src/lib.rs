use std::ops::Deref;

pub trait StrExt {
    fn to_i32(&self) -> i32;
}

impl<T> StrExt for T
where
    T: Deref<Target = str>,
{
    fn to_i32(&self) -> i32 {
        self.parse::<i32>().unwrap()
    }
}
