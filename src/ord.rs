use std::cmp::Ordering;

use crate::Avltriee;

pub trait AvltrieeOrd<T, I, A>: AsRef<Avltriee<T, I, A>>
where
    I: ?Sized,
{
    fn cmp(&self, left: &T, right: &I) -> Ordering;
}
