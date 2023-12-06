use rand::distributions::Bernoulli;
use rand::Rng;
use rand::{prelude::Distribution, thread_rng};

#[allow(dead_code)]
fn select_random_naive(nums: &[i32]) -> i32 {
    let mut rng = thread_rng();
    let mut candi = 0;
    for (idx, e) in nums.iter().enumerate() {
        let bnl = Bernoulli::new(1.0 / (idx + 1) as f64).unwrap();
        if bnl.sample(&mut rng) {
            candi = *e;
        }
    }
    candi
}

pub fn select_random<T: IntoIterator>(nums: T) -> Option<T::Item> {
    let mut rng = thread_rng();
    let mut candi = None;
    for (idx, e) in nums.into_iter().enumerate() {
        let bnl = match Bernoulli::new(1.0 / (idx + 1) as f64) {
            Ok(v) => v,
            Err(e) => {
                panic!("error occurs when getting bernoulli distribution: {}", e);
            }
        };

        if bnl.sample(&mut rng) {
            candi = Some(e);
        }
    }
    candi
}

// if pool_sz is longer than nums, then the result may not have the same length as pool_sz required
// mathmetical prove below:
// let pool size is k
// when i <= k, P(choose ith) = 1
// when i > k, for ith element, P(choose ith) = k / i, and swith ith with random element in pool
// then for i+1th element, P(choose i+1th) = k / (i+1), prove that P(chhoose ith) = k / (i+1)
// P(choose ith) = k / i * (P(not choose i+1th) + P(choose i+1th, but not change with ith))

pub fn sample_pool<I: IntoIterator>(nums: I, pool_sz: usize) -> Option<Vec<I::Item>> {
    let mut rng = thread_rng();
    let mut pool = Vec::new();

    for (idx, e) in nums.into_iter().enumerate() {
        if idx < pool_sz {
            pool.push(e);
            continue;
        }

        let bnl = match Bernoulli::new(pool_sz as f64 / (idx + 1) as f64) {
            Ok(v) => v,
            Err(e) => {
                panic!("error occurs when getting bernoulli distribution: {}", e);
            }
        };

        if bnl.sample(&mut rng) {
            let idx = rng.gen_range(0..pool_sz);
            pool[idx] = e;
        }
    }

    if !pool.is_empty() {
        Some(pool)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;
    use std::iter;

    #[test]
    fn test_select_random_basic() {
        let v = vec![1, 2, 3, 4, 5, 6];

        iter::repeat_with(|| println!("select {:?}", select_random(&v)))
            .take(10)
            .last();
    }

    #[test]
    fn test_select_random_uniform() {
        let v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut cnt = [0; 10];
        const EXPERI: usize = 100000;

        for _ in 0..EXPERI {
            let idx = *select_random(&v).unwrap() as usize;
            assert!((1..=10).contains(&idx));
            cnt[idx - 1] += 1;
        }

        cnt.iter()
            .enumerate()
            .map(|(idx, x)| println!("{}: {:.4}", idx, *x as f64 / EXPERI as f64 * 100.0))
            .last();
    }

    #[test]
    fn select_with_another_random() {
        let mut rng = thread_rng();
        let v = [1, 2, 3, 4, 5, 6];
        let mut cnt = [0; 6];
        let mut cnt_test = [0; 6];
        const EXPERI: i32 = 100000;

        for _ in 0..EXPERI {
            {
                let idx = *select_random(&v).unwrap() as usize;
                cnt[idx - 1] += 1;
            }
            {
                let idx = rng.gen_range(0..v.len());
                cnt_test[idx] += 1;
            }
        }

        cnt.iter()
            .enumerate()
            .map(|(idx, x)| println!("{}: {:.4}", idx, *x as f64 / EXPERI as f64 * 100.0))
            .last();

        println!("===========");

        cnt_test
            .iter()
            .enumerate()
            .map(|(idx, x)| println!("{}: {:.4}", idx, *x as f64 / EXPERI as f64 * 100.0))
            .last();

        for (x, y) in cnt.iter().zip(cnt_test.iter()) {
            let abrr: i32 = *x - *y;
            println!("aberration: {}", abrr);
            assert!((abrr.abs() as f64 / EXPERI as f64) < 0.005);
        }
    }

    #[test]
    fn test_build_random_pool() {
        let v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let pool_sz = 5;

        let res = sample_pool(&v, pool_sz);
        assert!(res.is_some());
        assert_eq!(res.as_ref().unwrap().len(), pool_sz);
        println!("{:?}", res);

        // test mutltiple time
        const EXPERIMENT: usize = 10000;
        let mut cnt = [0; 10];
        for _ in 0..EXPERIMENT {
            let res = sample_pool(&v, pool_sz);
            assert!(res.is_some());
            assert_eq!(res.as_ref().unwrap().len(), pool_sz);
            res.as_ref()
                .unwrap()
                .iter()
                .map(|&&x| cnt[x - 1] += 1)
                .last();
        }

        let mean = cnt.iter().sum::<usize>() / 10;
        println!("mean: {mean}");

        println!("{cnt:?}");
    }
}
