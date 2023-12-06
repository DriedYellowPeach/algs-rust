//! Quick-Find implementation of union-find data structure:
//! - fast find: O(1)
//! - slow union: depneds on the size of the union-find set

use super::UnionBeforeInsert;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub struct QuickFind<T> {
    // ids holds a hasmap of Value to ID
    item_ids: HashMap<T, usize>,
    // links holds the relationships of the union-find set
    relations: Vec<usize>,
}

impl<T> QuickFind<T>
where
    T: Debug + Hash + Eq,
{
    pub fn new() -> Self {
        QuickFind {
            item_ids: HashMap::new(),
            relations: Vec::new(),
        }
    }

    fn get_id(&self, item: &T) -> Option<usize> {
        self.item_ids.get(item).copied()
    }

    /// insert a new item, if the item already exists, do nothing
    pub fn insert(&mut self, item: T) {
        let new_id = self.relations.len();
        self.item_ids.entry(item).or_insert(new_id);
        self.relations.push(new_id);
    }

    pub fn size(&self) -> usize {
        self.relations.len()
    }

    pub fn is_connected(&self, p: &T, q: &T) -> bool {
        match (self.get_id(p), self.get_id(q)) {
            (Some(p_id), Some(q_id)) => self.relations[p_id] == self.relations[q_id],
            _ => false,
        }
    }

    pub fn union(&mut self, p: &T, q: &T) -> Result<(), UnionBeforeInsert> {
        if let (Some(p_id), Some(q_id)) = (self.get_id(p), self.get_id(q)) {
            for parent in self.relations.iter_mut() {
                if *parent == p_id {
                    *parent = q_id;
                }
            }
            Ok(())
        } else {
            Err(UnionBeforeInsert)
        }
    }
}

impl<T> Default for QuickFind<T>
where
    T: Debug + Eq + Hash,
{
    fn default() -> Self {
        QuickFind::new()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_quick_find() {
        let mut qf = QuickFind::new();
        qf.insert(5);
        qf.insert(4);
        qf.insert(6);
        assert!(qf.union(&5, &6).is_ok());
        assert!(qf.is_connected(&5, &6));
        assert!(!qf.is_connected(&4, &6));
    }
}
