use std::cmp::Ordering;

use anyhow::Result;

use super::{Avltriee, Found};

pub trait UOrd<T, I> {
    fn triee(&self) -> &Avltriee<T>;
    fn triee_mut(&mut self) -> &mut Avltriee<T>;
    fn cmp(&self, left: &T, right: &I) -> Ordering;
    fn search(&self, input: &I) -> Found;
    fn value(&mut self, input: &I) -> Result<T>;
    fn delete(&mut self, row: u32, delete_node: &T) -> Result<()>;
}

impl<T> Avltriee<T> {
    pub unsafe fn update_uord<I, U>(uord: &mut U, row: u32, input: I) -> Result<()>
    where
        T: Clone,
        U: UOrd<T, I>,
    {
        if let Some(n) = uord.triee().node(row) {
            let value = &n.value;
            if uord.cmp(value, &input) == Ordering::Equal {
                return Ok(()); //update value eq exists value
            }
            uord.delete(row, value)?;
        }
        let found = uord.search(&input);
        if found.ord == Ordering::Equal && found.row != 0 {
            uord.triee_mut().update_same(row, found.row);
        } else {
            let value = uord.value(&input)?;
            uord.triee_mut().update_unique(row, value, found);
        }
        Ok(())
    }
}
