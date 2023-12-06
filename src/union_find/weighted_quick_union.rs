use super::UnionBeforeInsert;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub struct Weighted_Quick_Union<T> {
    item_to_id: HashMap<T, usize>,
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl<T> Default for Weighted_Quick_Union<T> {
    fn default() -> Self {
        Weighted_Quick_Union {
            item_to_id: HashMap::new(),
            parents: Vec::new(),
            sizes: Vec::new(),
        }
    }
}

impl<T> Weighted_Quick_Union<T>
where
    T: Debug + Hash + Eq,
{
    pub fn new() -> Self {
        Weighted_Quick_Union::default()
    }

    pub fn insert(&mut self, item: T) {
        let new_id = self.parents.len();
        self.item_to_id.entry(item).or_insert(new_id);
        self.parents.push(new_id);
        self.sizes.push(1);
    }

    fn root_of(&self, child_id: usize) -> usize {
        let mut cur_id = child_id;
        while self.parents[cur_id] != cur_id {
            cur_id = self.parents[cur_id];
        }

        cur_id
    }

    // the size of the union
    pub fn size(&self) -> usize {
        self.parents.len()
    }

    pub fn is_connected(&self, p: &T, q: &T) -> bool {
        let Some(pid) = self.item_to_id.get(p) else {
            return false;
        };

        let Some(qid) = self.item_to_id.get(q) else {
            return false;
        };

        self.root_of(*pid) == self.root_of(*qid)
    }

    pub fn union(&mut self, p: &T, q: &T) -> Result<(), UnionBeforeInsert> {
        let Some(pid) = self.item_to_id.get(p) else {
            return Err(UnionBeforeInsert);
        };

        let Some(qid) = self.item_to_id.get(q) else {
            return Err(UnionBeforeInsert);
        };

        let proot = self.root_of(*pid);
        let qroot = self.root_of(*qid);

        if self.sizes[proot] > self.sizes[qroot] {
            self.parents[qroot] = proot;
            self.sizes[proot] += self.sizes[qroot];
        } else {
            self.parents[proot] = qroot;
            self.sizes[qroot] += self.sizes[proot];
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_weighted_quick_union_basics() {
        let mut wqu = Weighted_Quick_Union::new();
        wqu.insert(0);
        wqu.insert(1);
        wqu.insert(2);
        wqu.insert(3);
        wqu.insert(4);

        assert!(wqu.union(&0, &1).is_ok());
        assert!(wqu.union(&1, &2).is_ok());

        assert_eq!(wqu.parents[0], 1);
        assert_eq!(wqu.parents[1], 1);
        assert_eq!(wqu.parents[2], 1);

        assert!(wqu.union(&2, &3).is_ok());
        assert!(wqu.union(&3, &4).is_ok());

        assert_eq!(wqu.parents[3], 1);
        assert_eq!(wqu.parents[4], 1);

        assert_eq!(wqu.sizes, [1, 5, 1, 1, 1]);
    }
}
