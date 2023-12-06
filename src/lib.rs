pub mod priority_queue;
pub mod queue;
pub mod rand_sample;
pub mod sort;
pub mod stack;
pub mod union_find;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find_mod() {
        use union_find::QuickFind;
        let qf = QuickFind::<usize>::new();
        assert_eq!(qf.size(), 0);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
