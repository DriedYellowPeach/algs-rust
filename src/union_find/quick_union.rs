//! Quick-Union implementation of union-find data structure:
//! - find: depend on the height of the tree, O(h)
//! - union: also dpend on the height of the tree, O(h)
//! - tree can get taller and taller

use super::UnionBeforeInsert;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

// maybe derive Copy is not a good idea because if T is not a Copy, we can still have default method
pub struct QuickUnion<T> {
    // ids holds a hasmap of Value to ID
    item_ids: HashMap<T, usize>,
    // links: the <id>th item, its parent is parents[<id>], its parent is itself when the item is
    // the root
    parents: Vec<usize>,
}

impl<T> QuickUnion<T>
where
    T: Debug + Hash + Eq,
{
    pub fn new() -> Self {
        QuickUnion::default()
    }

    pub fn insert(&mut self, item: T) {
        let new_id = self.parents.len();
        self.item_ids.entry(item).or_insert(new_id);
        self.parents.push(new_id);
    }

    fn root_of(&self, child_id: usize) -> usize {
        let mut cur_id = child_id;
        while self.parents[cur_id] != cur_id {
            cur_id = self.parents[cur_id];
        }

        cur_id
    }

    pub fn size(&self) -> usize {
        self.parents.len()
    }

    pub fn is_connected(&self, p: &T, q: &T) -> bool {
        let Some(pid) = self.item_ids.get(p) else {
            // item p is even been inserted, so p, q are not connected
            return false;
        };

        let Some(qid) = self.item_ids.get(q) else {
            // item q is even been inserted, so p, q are not connected
            return false;
        };

        self.root_of(*pid) == self.root_of(*qid)
    }

    pub fn union(&mut self, p: &T, q: &T) -> Result<(), UnionBeforeInsert> {
        let Some(pid) = self.item_ids.get(p) else {
            // item p is not even been inserted, union error
            return Err(UnionBeforeInsert);
        };

        let Some(qid) = self.item_ids.get(q) else {
            // item q is not even been inserted, union error
            return Err(UnionBeforeInsert);
        };

        let proot = self.root_of(*pid);
        self.parents[proot] = self.root_of(*qid);
        Ok(())
    }
}

impl<T> Default for QuickUnion<T> {
    fn default() -> Self {
        QuickUnion {
            item_ids: HashMap::new(),
            parents: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_quick_uinon_basics() {
        let mut qu = QuickUnion::new();
        qu.insert(5);
        qu.insert(4);
        qu.insert(6);
        assert!(qu.union(&5, &6).is_ok());
        assert!(qu.is_connected(&5, &6));

        assert!(!qu.is_connected(&4, &6));

        assert!(qu.union(&4, &5).is_ok());
        assert!(qu.is_connected(&4, &5));
        assert!(qu.is_connected(&4, &6));
        assert_eq!(
            qu.parents[qu.item_ids.get(&4).copied().unwrap()],
            qu.item_ids.get(&6).copied().unwrap()
        );
    }
}
